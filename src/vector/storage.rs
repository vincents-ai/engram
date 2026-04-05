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

    #[test]
    #[should_panic]
    fn test_cosine_similarity_different_lengths() {
        let a = vec![1.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        cosine_similarity(&a, &b);
    }

    #[test]
    #[should_panic]
    fn test_euclidean_distance_different_lengths() {
        let a = vec![1.0];
        let b = vec![1.0, 2.0];
        euclidean_distance(&a, &b);
    }

    #[test]
    fn test_cosine_similarity_zero_vectors() {
        let a = vec![0.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert!((cosine_similarity(&a, &b) - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_cosine_similarity_negative() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![-1.0, 0.0, 0.0];
        assert!((cosine_similarity(&a, &b) - (-1.0)).abs() < 0.001);
    }

    #[test]
    fn test_cosine_similarity_acute_angle() {
        let a = vec![1.0, 1.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        let expected: f32 = 1.0 / (2.0_f32).sqrt();
        assert!((cosine_similarity(&a, &b) - expected).abs() < 0.001);
    }

    #[test]
    fn test_euclidean_distance_single_dimension() {
        let a = vec![5.0];
        let b = vec![3.0];
        assert!((euclidean_distance(&a, &b) - 2.0).abs() < 0.001);
    }

    #[test]
    fn test_euclidean_distance_negative_coords() {
        let a = vec![-1.0, -1.0];
        let b = vec![1.0, 1.0];
        let expected = (8.0_f32).sqrt();
        assert!((euclidean_distance(&a, &b) - expected).abs() < 0.001);
    }

    #[test]
    fn test_embedding_record_creation() {
        let now = Utc::now();
        let record = EmbeddingRecord {
            id: "rec-1".to_string(),
            entity_id: "entity-1".to_string(),
            entity_type: "task".to_string(),
            vector: vec![0.1, 0.2, 0.3],
            model: "test-model".to_string(),
            dimensions: 3,
            created_at: now,
        };

        assert_eq!(record.id, "rec-1");
        assert_eq!(record.entity_id, "entity-1");
        assert_eq!(record.vector.len(), 3);
        assert_eq!(record.dimensions, 3);
    }

    #[test]
    fn test_search_query_default() {
        let query = SearchQuery::default();
        assert!(query.text.is_empty());
        assert!(query.entity_types.is_empty());
        assert_eq!(query.limit, 10);
        assert!((query.threshold - 0.7).abs() < 0.001);
    }

    #[test]
    fn test_search_result_creation() {
        let result = SearchResult {
            entity_id: "e1".to_string(),
            entity_type: "task".to_string(),
            score: 0.95,
            snippet: Some("snippet text".to_string()),
            model: Some("model-1".to_string()),
        };

        assert_eq!(result.entity_id, "e1");
        assert_eq!(result.entity_type, "task");
        assert!((result.score - 0.95).abs() < 0.001);
        assert_eq!(result.snippet, Some("snippet text".to_string()));
    }

    #[test]
    fn test_search_result_minimal() {
        let result = SearchResult {
            entity_id: "e2".to_string(),
            entity_type: "context".to_string(),
            score: 0.5,
            snippet: None,
            model: None,
        };

        assert!(result.snippet.is_none());
        assert!(result.model.is_none());
    }
}
