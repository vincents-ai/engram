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
}
