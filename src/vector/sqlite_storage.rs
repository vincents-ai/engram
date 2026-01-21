use super::storage::cosine_similarity;
use super::SearchResult;
use anyhow::{Context, Result};
use rusqlite::{params, Connection, OptionalExtension};
use std::path::Path;
use std::sync::{Arc, Mutex};

pub struct SqliteVectorStorage {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteVectorStorage {
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let conn = Connection::open(db_path.as_ref()).context("Failed to open SQLite database")?;

        unsafe {
            rusqlite::ffi::sqlite3_auto_extension(Some(std::mem::transmute(
                sqlite_vec::sqlite3_vec_init as *const (),
            )));
        }

        let storage = Self {
            conn: Arc::new(Mutex::new(conn)),
        };

        storage.init_schema()?;
        Ok(storage)
    }

    pub fn memory() -> Result<Self> {
        let conn = Connection::open_in_memory().context("Failed to open in-memory database")?;

        unsafe {
            rusqlite::ffi::sqlite3_auto_extension(Some(std::mem::transmute(
                sqlite_vec::sqlite3_vec_init as *const (),
            )));
        }

        let storage = Self {
            conn: Arc::new(Mutex::new(conn)),
        };

        storage.init_schema()?;
        Ok(storage)
    }

    fn init_schema(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS embeddings (
                id TEXT PRIMARY KEY,
                entity_id TEXT NOT NULL,
                entity_type TEXT NOT NULL,
                vector BLOB NOT NULL,
                model TEXT NOT NULL,
                dimensions INTEGER NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                UNIQUE(entity_id, model)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS models (
                name TEXT PRIMARY KEY,
                provider TEXT NOT NULL,
                dimensions INTEGER NOT NULL,
                is_default BOOLEAN DEFAULT 0,
                config TEXT
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS search_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                query TEXT NOT NULL,
                results TEXT NOT NULL,
                timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_embeddings_entity 
             ON embeddings(entity_id, entity_type)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_embeddings_model 
             ON embeddings(model)",
            [],
        )?;

        Ok(())
    }

    pub fn store_embedding(
        &self,
        entity_id: &str,
        entity_type: &str,
        embedding: &[f32],
        model: &str,
    ) -> Result<()> {
        let id = uuid::Uuid::new_v4().to_string();
        let dimensions = embedding.len();
        let vector_bytes = bytemuck::cast_slice::<f32, u8>(embedding);

        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO embeddings 
             (id, entity_id, entity_type, vector, model, dimensions)
             VALUES (?, ?, ?, ?, ?, ?)",
            params![
                id,
                entity_id,
                entity_type,
                vector_bytes,
                model,
                dimensions as i64
            ],
        )?;

        Ok(())
    }

    pub fn get_embedding(&self, entity_id: &str, model: &str) -> Result<Option<Vec<f32>>> {
        let conn = self.conn.lock().unwrap();

        let result: Option<Vec<u8>> = conn
            .query_row(
                "SELECT vector FROM embeddings 
                 WHERE entity_id = ? AND model = ?",
                params![entity_id, model],
                |row| row.get(0),
            )
            .optional()?;

        Ok(result.map(|bytes| bytemuck::cast_slice::<u8, f32>(&bytes).to_vec()))
    }

    pub fn delete_embedding(&self, entity_id: &str, model: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "DELETE FROM embeddings WHERE entity_id = ? AND model = ?",
            params![entity_id, model],
        )?;
        Ok(())
    }

    pub fn search_similar(
        &self,
        query_embedding: &[f32],
        entity_type: Option<&str>,
        limit: usize,
        threshold: f32,
    ) -> Result<Vec<SearchResult>> {
        let conn = self.conn.lock().unwrap();

        let type_filter = if let Some(et) = entity_type {
            format!("AND entity_type = '{}'", et)
        } else {
            String::new()
        };

        let mut stmt = conn.prepare(&format!(
            "SELECT entity_id, entity_type, vector, model
             FROM embeddings
             WHERE 1=1 {}
             LIMIT ?",
            type_filter
        ))?;

        let results = stmt.query_map(params![limit * 10], |row| {
            let entity_id: String = row.get(0)?;
            let entity_type: String = row.get(1)?;
            let vector_bytes: Vec<u8> = row.get(2)?;
            let model: String = row.get(3)?;

            let embedding = bytemuck::cast_slice::<u8, f32>(&vector_bytes).to_vec();
            let score = cosine_similarity(query_embedding, &embedding);

            Ok(SearchResult {
                entity_id,
                entity_type,
                score,
                snippet: None,
                model: Some(model),
            })
        })?;

        let mut filtered: Vec<SearchResult> = results
            .filter_map(|r| r.ok())
            .filter(|r| r.score >= threshold)
            .collect();

        filtered.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        filtered.truncate(limit);

        Ok(filtered)
    }

    pub fn register_model(
        &self,
        name: &str,
        provider: &str,
        dimensions: usize,
        is_default: bool,
    ) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        if is_default {
            conn.execute("UPDATE models SET is_default = 0", [])?;
        }

        conn.execute(
            "INSERT OR REPLACE INTO models (name, provider, dimensions, is_default)
             VALUES (?, ?, ?, ?)",
            params![name, provider, dimensions as i64, is_default],
        )?;

        Ok(())
    }

    pub fn get_default_model(&self) -> Result<Option<String>> {
        let conn = self.conn.lock().unwrap();
        let result = conn
            .query_row("SELECT name FROM models WHERE is_default = 1", [], |row| {
                row.get(0)
            })
            .optional()?;
        Ok(result)
    }

    pub fn count_embeddings(&self) -> Result<usize> {
        let conn = self.conn.lock().unwrap();
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM embeddings", [], |row| row.get(0))?;
        Ok(count as usize)
    }

    pub fn list_models(&self) -> Result<Vec<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT name FROM models")?;
        let models: Result<Vec<String>, rusqlite::Error> =
            stmt.query_map([], |row| row.get(0))?.collect();
        models.map_err(|e| anyhow::anyhow!("Failed to list models: {}", e).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector::embedding::MockEmbeddingProvider;

    #[test]
    fn test_sqlite_storage_init() {
        let storage = SqliteVectorStorage::memory().unwrap();
        assert_eq!(storage.count_embeddings().unwrap(), 0);
    }

    #[test]
    fn test_store_and_retrieve() {
        let storage = SqliteVectorStorage::memory().unwrap();
        let embedding = vec![0.1, 0.2, 0.3];

        storage
            .store_embedding("entity1", "task", &embedding, "test-model")
            .unwrap();

        let retrieved = storage.get_embedding("entity1", "test-model").unwrap();
        assert!(retrieved.is_some());

        let retrieved_vec = retrieved.unwrap();
        assert_eq!(retrieved_vec.len(), 3);
        assert!((retrieved_vec[0] - 0.1).abs() < 0.001);
    }

    #[test]
    fn test_search_similar() {
        let storage = SqliteVectorStorage::memory().unwrap();

        storage
            .store_embedding("e1", "task", &[1.0, 0.0, 0.0], "test")
            .unwrap();
        storage
            .store_embedding("e2", "task", &[0.9, 0.1, 0.0], "test")
            .unwrap();
        storage
            .store_embedding("e3", "task", &[0.0, 1.0, 0.0], "test")
            .unwrap();

        let query = vec![1.0, 0.0, 0.0];
        let results = storage
            .search_similar(&query, Some("task"), 10, 0.5)
            .unwrap();

        assert!(results.len() >= 2);
        assert_eq!(results[0].entity_id, "e1");
        assert!(results[0].score > 0.9);
    }

    #[test]
    fn test_model_registration() {
        let storage = SqliteVectorStorage::memory().unwrap();

        storage
            .register_model("model1", "local", 384, true)
            .unwrap();
        storage.register_model("model2", "api", 512, false).unwrap();

        let default = storage.get_default_model().unwrap();
        assert_eq!(default, Some("model1".to_string()));
    }

    #[test]
    fn test_delete_embedding() {
        let storage = SqliteVectorStorage::memory().unwrap();
        let embedding = vec![0.1, 0.2, 0.3];

        storage
            .store_embedding("entity1", "task", &embedding, "test")
            .unwrap();
        assert!(storage.get_embedding("entity1", "test").unwrap().is_some());

        storage.delete_embedding("entity1", "test").unwrap();
        assert!(storage.get_embedding("entity1", "test").unwrap().is_none());
    }
}
