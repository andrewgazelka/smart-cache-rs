use std::path::PathBuf;

pub use smart_cache_macro::cached;

use eyre::Result;
use once_cell::sync::Lazy;
use redb::{Database, TableDefinition};
use tracing::{debug, trace};

// Define the table that will store our cache entries
const CACHE_TABLE: TableDefinition<&[u8], &[u8]> = TableDefinition::new("cache");

static DB: Lazy<Database> = Lazy::new(|| {
    let cache_dir = dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from(".cache"))
        .join("smart-cache");
    std::fs::create_dir_all(&cache_dir).expect("failed to create cache directory");

    let db_path = cache_dir.join("cache.redb");
    Database::create(db_path).expect("failed to create cache database")
});

/// Internal function used by the macro to get a cached value
#[doc(hidden)]
pub fn get_cached(key_bytes: &[u8]) -> Option<Vec<u8>> {
    trace!("Attempting cache lookup");

    match DB.begin_read() {
        Ok(txn) => match txn.open_table(CACHE_TABLE) {
            Ok(table) => match table.get(key_bytes) {
                Ok(Some(value)) => {
                    debug!("Cache hit");
                    Some(value.value().to_vec())
                }
                Ok(None) => {
                    debug!("Cache miss");
                    None
                }
                Err(e) => {
                    debug!("Cache error: {}", e);
                    None
                }
            },
            Err(e) => {
                debug!("Failed to open table: {}", e);
                None
            }
        },
        Err(e) => {
            debug!("Failed to begin read transaction: {}", e);
            None
        }
    }
}

/// Internal function used by the macro to set a cached value
#[doc(hidden)]
pub fn set_cached(key: &[u8], value: &[u8]) -> Result<()> {
    trace!("Caching value");

    let write_txn = DB.begin_write()?;
    {
        let mut table = write_txn.open_table(CACHE_TABLE)?;
        table.insert(key, value)?;
    }
    write_txn.commit()?;

    debug!("Successfully cached value");
    Ok(())
}
