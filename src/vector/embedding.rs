//! Embedding generation traits and providers

use super::Result;
use async_trait::async_trait;

#[async_trait]
pub trait EmbeddingProvider: Send + Sync {
    async fn embed(&self, text: &str) -> Result<Vec<f32>>;

    async fn embed_batch(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>> {
        let mut embeddings = Vec::with_capacity(texts.len());
        for text in texts {
            embeddings.push(self.embed(text).await?);
        }
        Ok(embeddings)
    }

    fn dimensions(&self) -> usize;

    fn model_name(&self) -> &str;

    fn provider_type(&self) -> ProviderType;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderType {
    LocalOnnx,
    LocalCandle,
    OpenAI,
    Cohere,
    Voyage,
    Mock,
}

impl std::fmt::Display for ProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProviderType::LocalOnnx => write!(f, "local_onnx"),
            ProviderType::LocalCandle => write!(f, "local_candle"),
            ProviderType::OpenAI => write!(f, "openai"),
            ProviderType::Cohere => write!(f, "cohere"),
            ProviderType::Voyage => write!(f, "voyage"),
            ProviderType::Mock => write!(f, "mock"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MockEmbeddingProvider {
    dimensions: usize,
    model_name: String,
}

impl MockEmbeddingProvider {
    pub fn new(dimensions: usize) -> Self {
        Self {
            dimensions,
            model_name: "mock-embeddings".to_string(),
        }
    }
}

#[async_trait]
impl EmbeddingProvider for MockEmbeddingProvider {
    async fn embed(&self, text: &str) -> Result<Vec<f32>> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        let hash = hasher.finish();

        let mut vec = Vec::with_capacity(self.dimensions);
        let mut rng = hash;
        for _ in 0..self.dimensions {
            rng = rng.wrapping_mul(1103515245).wrapping_add(12345);
            vec.push(((rng as f32) / (u64::MAX as f32)) * 2.0 - 1.0);
        }

        let norm: f32 = vec.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for val in vec.iter_mut() {
                *val /= norm;
            }
        }

        Ok(vec)
    }

    fn dimensions(&self) -> usize {
        self.dimensions
    }

    fn model_name(&self) -> &str {
        &self.model_name
    }

    fn provider_type(&self) -> ProviderType {
        ProviderType::Mock
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_provider() {
        let provider = MockEmbeddingProvider::new(384);

        let embedding = provider.embed("test text").await.unwrap();
        assert_eq!(embedding.len(), 384);

        let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 0.001);
    }

    #[tokio::test]
    async fn test_mock_batch() {
        let provider = MockEmbeddingProvider::new(128);

        let texts = vec!["text1", "text2", "text3"];
        let embeddings = provider.embed_batch(&texts).await.unwrap();

        assert_eq!(embeddings.len(), 3);
        for emb in embeddings {
            assert_eq!(emb.len(), 128);
        }
    }

    #[tokio::test]
    async fn test_deterministic() {
        let provider = MockEmbeddingProvider::new(256);

        let emb1 = provider.embed("same text").await.unwrap();
        let emb2 = provider.embed("same text").await.unwrap();

        assert_eq!(emb1, emb2);
    }

    #[tokio::test]
    async fn test_mock_provider_different_texts() {
        let provider = MockEmbeddingProvider::new(64);

        let emb1 = provider.embed("text A").await.unwrap();
        let emb2 = provider.embed("text B").await.unwrap();

        assert_ne!(emb1, emb2);
    }

    #[tokio::test]
    async fn test_mock_provider_dimensions() {
        let provider = MockEmbeddingProvider::new(10);
        assert_eq!(provider.dimensions(), 10);
    }

    #[tokio::test]
    async fn test_mock_provider_model_name() {
        let provider = MockEmbeddingProvider::new(128);
        assert_eq!(provider.model_name(), "mock-embeddings");
    }

    #[tokio::test]
    async fn test_mock_provider_type() {
        let provider = MockEmbeddingProvider::new(64);
        assert_eq!(provider.provider_type(), ProviderType::Mock);
    }

    #[tokio::test]
    async fn test_mock_batch_empty() {
        let provider = MockEmbeddingProvider::new(64);
        let texts: Vec<&str> = vec![];
        let embeddings = provider.embed_batch(&texts).await.unwrap();
        assert!(embeddings.is_empty());
    }

    #[tokio::test]
    async fn test_mock_provider_single_dimension() {
        let provider = MockEmbeddingProvider::new(1);
        let embedding = provider.embed("test").await.unwrap();
        assert_eq!(embedding.len(), 1);
        let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_provider_type_display() {
        assert_eq!(format!("{}", ProviderType::LocalOnnx), "local_onnx");
        assert_eq!(format!("{}", ProviderType::LocalCandle), "local_candle");
        assert_eq!(format!("{}", ProviderType::OpenAI), "openai");
        assert_eq!(format!("{}", ProviderType::Cohere), "cohere");
        assert_eq!(format!("{}", ProviderType::Voyage), "voyage");
        assert_eq!(format!("{}", ProviderType::Mock), "mock");
    }

    #[test]
    fn test_provider_type_equality() {
        assert_eq!(ProviderType::LocalOnnx, ProviderType::LocalOnnx);
        assert_ne!(ProviderType::LocalOnnx, ProviderType::OpenAI);
    }
}
