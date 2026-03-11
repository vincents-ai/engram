# API Reference

## Rust API

For Rust API documentation, see the source code in `src/`.

### Core Traits

```rust
pub trait Entity {
    fn entity_type() -> &'static str;
    fn id(&self) -> &str;
    fn agent(&self) -> &str;
    fn timestamp(&self) -> DateTime<Utc>;
    fn validate_entity(&self) -> Result<()>;
    fn to_generic(&self) -> GenericEntity;
    fn from_generic(entity: GenericEntity) -> Result<Self>;
}
```

### Storage Trait

```rust
pub trait Storage {
    fn store(&mut self, entity: &GenericEntity) -> Result<()>;
    fn get(&self, id: &str, entity_type: &str) -> Result<Option<GenericEntity>>;
    fn list_ids(&self, entity_type: &str) -> Result<Vec<String>>;
    fn delete(&mut self, id: &str, entity_type: &str) -> Result<()>;
}
```
