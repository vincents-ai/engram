use super::embedding::{EmbeddingProvider, ProviderType};
use super::Result;
use crate::error::{EngramError, StorageError};
use async_trait::async_trait;
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use std::sync::Mutex;

pub struct FastEmbedProvider {
    model: Mutex<TextEmbedding>,
    model_name: String,
    dimensions: usize,
}

impl FastEmbedProvider {
    pub fn new() -> Result<Self> {
        Self::with_model(EmbeddingModel::AllMiniLML6V2)
    }

    pub fn with_model(model_type: EmbeddingModel) -> Result<Self> {
        let model = TextEmbedding::try_new(
            InitOptions::new(model_type.clone()).with_show_download_progress(true),
        )
        .map_err(|e| {
            EngramError::Storage(StorageError::InvalidState(format!(
                "Failed to initialize FastEmbed model: {}",
                e
            )))
        })?;

        let dimensions = match model_type {
            EmbeddingModel::AllMiniLML6V2 => 384,
            EmbeddingModel::AllMiniLML12V2 => 384,
            EmbeddingModel::BGESmallENV15 => 384,
            EmbeddingModel::BGEBaseENV15 => 768,
            EmbeddingModel::BGELargeENV15 => 1024,
            _ => 384,
        };

        let model_name = format!("{:?}", model_type);

        Ok(Self {
            model: Mutex::new(model),
            model_name,
            dimensions,
        })
    }
}

#[async_trait]
impl EmbeddingProvider for FastEmbedProvider {
    async fn embed(&self, text: &str) -> Result<Vec<f32>> {
        let mut model = self.model.lock().map_err(|e| {
            EngramError::Storage(StorageError::InvalidState(format!(
                "Failed to lock model: {}",
                e
            )))
        })?;

        let embeddings = model.embed(vec![text], None).map_err(|e| {
            EngramError::Storage(StorageError::InvalidState(format!(
                "Failed to generate embedding: {}",
                e
            )))
        })?;

        embeddings.into_iter().next().ok_or_else(|| {
            EngramError::Storage(StorageError::InvalidState(
                "No embedding generated".to_string(),
            ))
        })
    }

    async fn embed_batch(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>> {
        let mut model = self.model.lock().map_err(|e| {
            EngramError::Storage(StorageError::InvalidState(format!(
                "Failed to lock model: {}",
                e
            )))
        })?;

        model.embed(texts.to_vec(), None).map_err(|e| {
            EngramError::Storage(StorageError::InvalidState(format!(
                "Failed to generate batch embeddings: {}",
                e
            )))
        })
    }

    fn dimensions(&self) -> usize {
        self.dimensions
    }

    fn model_name(&self) -> &str {
        &self.model_name
    }

    fn provider_type(&self) -> ProviderType {
        ProviderType::LocalOnnx
    }
}

impl Default for FastEmbedProvider {
    fn default() -> Self {
        Self::new().expect("Failed to create default FastEmbed provider")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fastembed_provider_init() {
        let provider = FastEmbedProvider::new();
        assert!(provider.is_ok());

        let provider = provider.unwrap();
        assert_eq!(provider.dimensions(), 384);
    }

    #[tokio::test]
    async fn test_fastembed_embed() {
        let provider = FastEmbedProvider::new().unwrap();
        let embedding = provider.embed("Hello, world!").await;

        assert!(embedding.is_ok());
        let vec = embedding.unwrap();
        assert_eq!(vec.len(), 384);

        assert!(vec.iter().any(|&x| x != 0.0));
    }

    #[tokio::test]
    async fn test_fastembed_embed_batch() {
        let provider = FastEmbedProvider::new().unwrap();
        let texts = vec!["First text", "Second text", "Third text"];

        let embeddings = provider.embed_batch(&texts).await;
        assert!(embeddings.is_ok());

        let vecs = embeddings.unwrap();
        assert_eq!(vecs.len(), 3);
        assert!(vecs.iter().all(|v| v.len() == 384));
    }

    #[tokio::test]
    async fn test_deterministic_embeddings() {
        let provider = FastEmbedProvider::new().unwrap();
        let text = "Test text for determinism";

        let embedding1 = provider.embed(text).await.unwrap();
        let embedding2 = provider.embed(text).await.unwrap();

        assert_eq!(embedding1.len(), embedding2.len());

        for (a, b) in embedding1.iter().zip(embedding2.iter()) {
            assert!((a - b).abs() < 1e-6);
        }
    }
}
