//! IndexedDB access layer for persisted application data.

pub mod index_db;
pub use index_db::{Article, IndexedDbClient, Settings};
