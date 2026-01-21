//! Vector storage for embeddings and similarity search

use super::{Result, SearchQuery, SearchResult};
use chrono::{DateTime, Utc};

pub trait VectorStorage: Send + Sync {
    fn store_embedding(
        &mut self,
        entity_id: &str,
        entity_type: &str,
        vector: &[f32],
        model: &str,
    ) -> Result<()>;

    fn get_embedding(&self, entity_id: &str, model: &str) -> Result<Option<Vec<f32>>>;

    fn search(&self, query_vector: &[f32], query: &SearchQuery) -> Result<Vec<SearchResult>>;

    fn find_similar(
        &self,
        entity_id: &str,
        limit: usize,
        threshold: f32,
    ) -> Result<Vec<SearchResult>>;

    fn delete_embedding(&mut self, entity_id: &str, model: &str) -> Result<()>;

    fn count_embeddings(&self, model: Option<&str>) -> Result<usize>;

    fn list_models(&self) -> Result<Vec<String>>;
}

#[derive(Debug, Clone)]
pub struct EmbeddingRecord {
    pub id: String,
    pub entity_id: String,
    pub entity_type: String,
    pub vector: Vec<f32>,
    pub model: String,
    pub dimensions: usize,
    pub created_at: DateTime<Utc>,
}

pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len(), "Vectors must have same dimensions");

    let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }

    dot_product / (norm_a * norm_b)
}

pub fn euclidean_distance(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len(), "Vectors must have same dimensions");

    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f32>()
        .sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert!((cosine_similarity(&a, &b) - 1.0).abs() < 0.001);

        let a = vec![1.0, 0.0];
        let b = vec![0.0, 1.0];
        assert!((cosine_similarity(&a, &b) - 0.0).abs() < 0.001);

        let a = vec![1.0, 1.0];
        let b = vec![1.0, 1.0];
        assert!((cosine_similarity(&a, &b) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_euclidean_distance() {
        let a = vec![0.0, 0.0];
        let b = vec![3.0, 4.0];
        assert!((euclidean_distance(&a, &b) - 5.0).abs() < 0.001);

        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0, 2.0, 3.0];
        assert!((euclidean_distance(&a, &b) - 0.0).abs() < 0.001);
    }
}
