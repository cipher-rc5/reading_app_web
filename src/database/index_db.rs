use anyhow::{Result, anyhow};
use indexed_db_futures::KeyPath;
use indexed_db_futures::database::Database;
use indexed_db_futures::iter::ArrayMapIter;
use indexed_db_futures::prelude::*;
use indexed_db_futures::transaction::{Transaction, TransactionMode};
use serde::{Deserialize, Serialize};

const DB_NAME: &str = "ReadingApp";
const DB_VERSION: u32 = 1;
const ARTICLES_STORE: &str = "articles";
const SETTINGS_STORE: &str = "settings";
const PRIMARY_KEY: &str = "id";
const DEFAULT_SETTINGS_ID: &str = "default";

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Persisted reading article.
pub struct Article {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: i64,
    pub word_count: usize,
    pub reading_time_minutes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Persisted UI/user settings.
pub struct Settings {
    pub id: String,
    pub font_size: f32,
    pub font_family: String,
    pub theme: String,
}

/// Thin async client around IndexedDB object stores used by the app.
pub struct IndexedDbClient {
    db: Database,
}

impl IndexedDbClient {
    /// Opens the database and creates required stores on first run.
    pub async fn new() -> Result<Self> {
        let db = Database::open(DB_NAME)
            .with_version(DB_VERSION)
            .with_on_upgrade_needed(|_, db| {
                ensure_object_store(&db, ARTICLES_STORE)?;
                ensure_object_store(&db, SETTINGS_STORE)?;
                Ok(())
            })
            .await
            .map_err(|e| anyhow!("failed to open IndexedDB: {e}"))?;

        Ok(Self { db })
    }

    /// Inserts or updates an article by its `id` key.
    pub async fn save_article(&self, article: &Article) -> Result<()> {
        let tx = self.open_transaction(ARTICLES_STORE, TransactionMode::Readwrite)?;
        let store = tx
            .object_store(ARTICLES_STORE)
            .map_err(|e| anyhow!("failed to access `{ARTICLES_STORE}` store: {e}"))?;
        store
            .put(article)
            .serde()
            .map_err(|e| anyhow!("failed to serialise article: {e}"))?
            .await
            .map_err(|e| anyhow!("failed to store article: {e}"))?;

        tx.commit()
            .await
            .map_err(|e| anyhow!("failed to commit article transaction: {e}"))?;

        Ok(())
    }

    /// Loads all stored articles.
    pub async fn get_all_articles(&self) -> Result<Vec<Article>> {
        let tx = self.open_transaction(ARTICLES_STORE, TransactionMode::Readonly)?;
        let store = tx
            .object_store(ARTICLES_STORE)
            .map_err(|e| anyhow!("failed to access `{ARTICLES_STORE}` store: {e}"))?;
        let iter = store
            .get_all()
            .serde()
            .map_err(|e| anyhow!("failed to request article list: {e}"))?
            .await
            .map_err(|e| anyhow!("failed to load article list: {e}"))?;

        collect_results(iter, "failed to decode article list entries")
    }

    /// Deletes an article by id.
    pub async fn delete_article(&self, id: &str) -> Result<()> {
        let tx = self.open_transaction(ARTICLES_STORE, TransactionMode::Readwrite)?;
        let store = tx
            .object_store(ARTICLES_STORE)
            .map_err(|e| anyhow!("failed to access `{ARTICLES_STORE}` store: {e}"))?;
        store
            .delete(id)
            .serde()
            .map_err(|e| anyhow!("failed to prepare delete for article {id}: {e}"))?
            .await
            .map_err(|e| anyhow!("failed to delete article {id}: {e}"))?;

        tx.commit()
            .await
            .map_err(|e| anyhow!("failed to commit delete transaction: {e}"))?;

        Ok(())
    }

    /// Inserts or updates default settings (`id = "default"`).
    pub async fn save_settings(&self, settings: &Settings) -> Result<()> {
        let tx = self.open_transaction(SETTINGS_STORE, TransactionMode::Readwrite)?;
        let store = tx
            .object_store(SETTINGS_STORE)
            .map_err(|e| anyhow!("failed to access `{SETTINGS_STORE}` store: {e}"))?;
        store
            .put(settings)
            .serde()
            .map_err(|e| anyhow!("failed to serialise settings: {e}"))?
            .await
            .map_err(|e| anyhow!("failed to persist settings: {e}"))?;

        tx.commit()
            .await
            .map_err(|e| anyhow!("failed to commit settings transaction: {e}"))?;

        Ok(())
    }

    /// Loads the default settings record when present.
    pub async fn get_settings(&self) -> Result<Option<Settings>> {
        let tx = self.open_transaction(SETTINGS_STORE, TransactionMode::Readonly)?;
        let store = tx
            .object_store(SETTINGS_STORE)
            .map_err(|e| anyhow!("failed to access `{SETTINGS_STORE}` store: {e}"))?;
        let settings = store
            .get(DEFAULT_SETTINGS_ID)
            .serde()
            .map_err(|e| anyhow!("failed to request default settings: {e}"))?
            .await
            .map_err(|e| anyhow!("failed to read default settings: {e}"))?;

        Ok(settings)
    }

    fn open_transaction<'a>(
        &'a self,
        store_name: &str,
        mode: TransactionMode,
    ) -> Result<Transaction<'a>> {
        self.db
            .transaction(store_name)
            .with_mode(mode)
            .build()
            .map_err(|e| anyhow!("failed to start `{store_name}` transaction: {e}"))
    }
}

fn ensure_object_store(db: &Database, store_name: &str) -> indexed_db_futures::Result<()> {
    if db.object_store_names().any(|name| name == store_name) {
        return Ok(());
    }

    db.create_object_store(store_name)
        .with_key_path(KeyPath::from(PRIMARY_KEY))
        .build()
        .map(|_| ())
}

fn collect_results<T>(iter: ArrayMapIter<T>, label: &str) -> Result<Vec<T>> {
    iter.map(|entry| entry.map_err(|e| anyhow!("{label}: {e}")))
        .collect()
}
