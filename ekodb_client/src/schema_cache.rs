//! # Schema Cache
//!
//! Opt-in in-memory cache for collection schema metadata (primary_key_alias, fields, version).
//! Eliminates redundant schema API calls for ID extraction and field validation.
//!
//! ## Features
//!
//! - **LRU eviction**: bounded memory — oldest entries evicted when capacity exceeded
//! - **TTL expiry**: entries go stale after configurable duration (default 5 min)
//! - **Realtime invalidation**: `handle_schema_changed()` for WS/SSE push events
//! - **Thread-safe**: `Mutex`-protected `HashMap`/`VecDeque` (low contention, no extra deps)
//! - **Zero overhead when disabled**: all methods short-circuit

use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

/// Cached schema metadata for a single collection.
#[derive(Debug, Clone)]
pub struct CachedSchema {
    /// The collection's primary key field name (e.g. "id", "_id", "user_id")
    pub primary_key_alias: String,
    /// Schema version (increments on each schema change)
    pub version: u64,
    /// When this entry was cached
    pub cached_at: Instant,
}

/// Configuration for the schema cache.
#[derive(Debug, Clone)]
pub struct SchemaCacheConfig {
    /// Whether the cache is enabled (default: false)
    pub enabled: bool,
    /// Maximum number of collections to cache (default: 100)
    pub max_entries: usize,
    /// Time-to-live before an entry is considered stale (default: 5 min)
    pub ttl: Duration,
}

impl Default for SchemaCacheConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            max_entries: 100,
            ttl: Duration::from_secs(300),
        }
    }
}

/// Thread-safe, opt-in schema cache with LRU eviction and TTL expiry.
///
/// Uses `Mutex<HashMap>` instead of `DashMap` to avoid adding a dependency.
/// Client-side access patterns are read-heavy with rare writes and low contention,
/// so a single lock is fine. The lock is held only for fast HashMap operations.
pub struct SchemaCache {
    /// Cached entries + LRU order, protected by a single lock.
    /// Combining them avoids lock ordering issues.
    inner: Mutex<SchemaCacheInner>,
    /// Configuration
    config: SchemaCacheConfig,
    /// Fast path: skip all logic when disabled
    enabled: AtomicBool,
}

struct SchemaCacheInner {
    entries: HashMap<String, CachedSchema>,
    lru_order: VecDeque<String>,
}

impl SchemaCache {
    /// Create a new schema cache with the given configuration.
    pub fn new(config: SchemaCacheConfig) -> Self {
        let enabled = config.enabled;
        Self {
            inner: Mutex::new(SchemaCacheInner {
                entries: HashMap::with_capacity(config.max_entries),
                lru_order: VecDeque::with_capacity(config.max_entries),
            }),
            config,
            enabled: AtomicBool::new(enabled),
        }
    }

    /// Create a disabled cache (zero overhead — all methods return None/no-op).
    pub fn disabled() -> Self {
        Self::new(SchemaCacheConfig::default())
    }

    /// Check if the cache is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }

    /// Enable or disable the cache at runtime.
    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::Relaxed);
    }

    /// Get the cached primary_key_alias for a collection, if available and not stale.
    pub fn get_alias(&self, collection: &str) -> Option<String> {
        if !self.is_enabled() {
            return None;
        }
        let mut inner = self.inner.lock().ok()?;
        let entry = inner.entries.get(collection)?;
        if entry.cached_at.elapsed() > self.config.ttl {
            inner.entries.remove(collection);
            inner.lru_order.retain(|c| c != collection);
            return None;
        }
        let alias = entry.primary_key_alias.clone();
        Self::touch_lru_inner(&mut inner.lru_order, collection);
        Some(alias)
    }

    /// Get the full cached schema for a collection, if available and not stale.
    pub fn get(&self, collection: &str) -> Option<CachedSchema> {
        if !self.is_enabled() {
            return None;
        }
        let mut inner = self.inner.lock().ok()?;
        let entry = inner.entries.get(collection)?;
        if entry.cached_at.elapsed() > self.config.ttl {
            inner.entries.remove(collection);
            inner.lru_order.retain(|c| c != collection);
            return None;
        }
        let cloned = entry.clone();
        Self::touch_lru_inner(&mut inner.lru_order, collection);
        Some(cloned)
    }

    /// Insert or update a cached schema entry.
    pub fn insert(&self, collection: &str, primary_key_alias: String, version: u64) {
        if !self.is_enabled() {
            return;
        }
        let mut inner = match self.inner.lock() {
            Ok(g) => g,
            Err(_) => return,
        };

        let is_new = !inner.entries.contains_key(collection);

        inner.entries.insert(
            collection.to_string(),
            CachedSchema {
                primary_key_alias,
                version,
                cached_at: Instant::now(),
            },
        );

        if is_new {
            inner.lru_order.push_back(collection.to_string());
            // Evict oldest if over capacity
            while inner.lru_order.len() > self.config.max_entries {
                if let Some(oldest) = inner.lru_order.pop_front() {
                    inner.entries.remove(&oldest);
                }
            }
        } else {
            Self::touch_lru_inner(&mut inner.lru_order, collection);
        }
    }

    /// Invalidate (remove) the cached schema for a specific collection.
    pub fn invalidate(&self, collection: &str) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.entries.remove(collection);
            inner.lru_order.retain(|c| c != collection);
        }
    }

    /// Invalidate all cached schemas.
    pub fn invalidate_all(&self) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.entries.clear();
            inner.lru_order.clear();
        }
    }

    /// Handle a SchemaChanged event from the server (WS or SSE push).
    /// Updates the cache if the event has a newer version, or inserts if unknown.
    pub fn handle_schema_changed(&self, collection: &str, version: u64, primary_key_alias: &str) {
        if !self.is_enabled() {
            return;
        }
        let should_update = {
            let inner = match self.inner.lock() {
                Ok(g) => g,
                Err(_) => return,
            };
            inner
                .entries
                .get(collection)
                .map(|entry| version > entry.version)
                .unwrap_or(true)
        };
        if should_update {
            self.insert(collection, primary_key_alias.to_string(), version);
        }
    }

    /// Get the number of cached entries.
    pub fn len(&self) -> usize {
        self.inner.lock().map(|g| g.entries.len()).unwrap_or(0)
    }

    /// Check if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Move a collection to the back of the LRU order (most recently used).
    fn touch_lru_inner(lru_order: &mut VecDeque<String>, collection: &str) {
        if let Some(pos) = lru_order.iter().position(|c| c == collection) {
            lru_order.remove(pos);
        }
        lru_order.push_back(collection.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn enabled_config() -> SchemaCacheConfig {
        SchemaCacheConfig {
            enabled: true,
            max_entries: 3,
            ttl: Duration::from_secs(60),
        }
    }

    #[test]
    fn test_insert_and_get() {
        let cache = SchemaCache::new(enabled_config());
        cache.insert("users", "id".to_string(), 1);

        let entry = cache.get("users").unwrap();
        assert_eq!(entry.primary_key_alias, "id");
        assert_eq!(entry.version, 1);
    }

    #[test]
    fn test_get_alias() {
        let cache = SchemaCache::new(enabled_config());
        cache.insert("orders", "order_id".to_string(), 1);

        assert_eq!(cache.get_alias("orders"), Some("order_id".to_string()));
        assert_eq!(cache.get_alias("nonexistent"), None);
    }

    #[test]
    fn test_ttl_expiry() {
        let config = SchemaCacheConfig {
            enabled: true,
            max_entries: 10,
            ttl: Duration::from_millis(1), // 1ms TTL
        };
        let cache = SchemaCache::new(config);
        cache.insert("users", "id".to_string(), 1);

        // Wait for TTL to expire
        std::thread::sleep(Duration::from_millis(5));

        assert!(cache.get("users").is_none());
        assert!(cache.get_alias("users").is_none());
    }

    #[test]
    fn test_invalidate_single() {
        let cache = SchemaCache::new(enabled_config());
        cache.insert("users", "id".to_string(), 1);
        cache.insert("orders", "id".to_string(), 1);

        cache.invalidate("users");

        assert!(cache.get("users").is_none());
        assert!(cache.get("orders").is_some());
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_invalidate_all() {
        let cache = SchemaCache::new(enabled_config());
        cache.insert("users", "id".to_string(), 1);
        cache.insert("orders", "id".to_string(), 1);

        cache.invalidate_all();

        assert!(cache.is_empty());
    }

    #[test]
    fn test_lru_eviction() {
        let config = SchemaCacheConfig {
            enabled: true,
            max_entries: 2, // only 2 entries
            ttl: Duration::from_secs(60),
        };
        let cache = SchemaCache::new(config);

        cache.insert("a", "id".to_string(), 1);
        cache.insert("b", "id".to_string(), 1);
        // Cache full, inserting "c" should evict "a" (oldest)
        cache.insert("c", "id".to_string(), 1);

        assert!(cache.get("a").is_none(), "a should be evicted");
        assert!(cache.get("b").is_some());
        assert!(cache.get("c").is_some());
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_lru_touch_prevents_eviction() {
        let config = SchemaCacheConfig {
            enabled: true,
            max_entries: 2,
            ttl: Duration::from_secs(60),
        };
        let cache = SchemaCache::new(config);

        cache.insert("a", "id".to_string(), 1);
        cache.insert("b", "id".to_string(), 1);
        // Touch "a" — makes it most recently used
        let _ = cache.get("a");
        // Insert "c" — should evict "b" (now oldest), not "a"
        cache.insert("c", "id".to_string(), 1);

        assert!(cache.get("a").is_some(), "a was touched, should survive");
        assert!(cache.get("b").is_none(), "b should be evicted");
        assert!(cache.get("c").is_some());
    }

    #[test]
    fn test_handle_schema_changed_updates() {
        let cache = SchemaCache::new(enabled_config());
        cache.insert("users", "id".to_string(), 1);

        // Newer version should update
        cache.handle_schema_changed("users", 2, "user_id");
        let entry = cache.get("users").unwrap();
        assert_eq!(entry.primary_key_alias, "user_id");
        assert_eq!(entry.version, 2);
    }

    #[test]
    fn test_handle_schema_changed_ignores_older() {
        let cache = SchemaCache::new(enabled_config());
        cache.insert("users", "user_id".to_string(), 5);

        // Older version should be ignored
        cache.handle_schema_changed("users", 3, "id");
        let entry = cache.get("users").unwrap();
        assert_eq!(entry.primary_key_alias, "user_id"); // unchanged
        assert_eq!(entry.version, 5);
    }

    #[test]
    fn test_handle_schema_changed_inserts_new() {
        let cache = SchemaCache::new(enabled_config());

        // New collection — should insert
        cache.handle_schema_changed("new_coll", 1, "_id");
        let entry = cache.get("new_coll").unwrap();
        assert_eq!(entry.primary_key_alias, "_id");
    }

    #[test]
    fn test_disabled_cache_returns_none() {
        let cache = SchemaCache::disabled();
        cache.insert("users", "id".to_string(), 1); // no-op

        assert!(cache.get("users").is_none());
        assert!(cache.get_alias("users").is_none());
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_runtime_enable_disable() {
        let cache = SchemaCache::new(enabled_config());
        cache.insert("users", "id".to_string(), 1);
        assert!(cache.get("users").is_some());

        cache.set_enabled(false);
        assert!(cache.get("users").is_none()); // disabled, returns None

        cache.set_enabled(true);
        assert!(cache.get("users").is_some()); // re-enabled, entry still there
    }

    #[test]
    fn test_update_existing_entry() {
        let cache = SchemaCache::new(enabled_config());
        cache.insert("users", "id".to_string(), 1);
        cache.insert("users", "user_id".to_string(), 2);

        let entry = cache.get("users").unwrap();
        assert_eq!(entry.primary_key_alias, "user_id");
        assert_eq!(entry.version, 2);
        assert_eq!(cache.len(), 1); // no duplicate
    }

    #[test]
    fn test_concurrent_access() {
        use std::sync::Arc;

        let cache = Arc::new(SchemaCache::new(SchemaCacheConfig {
            enabled: true,
            max_entries: 100,
            ttl: Duration::from_secs(60),
        }));

        let mut handles = vec![];
        for i in 0..10 {
            let cache = cache.clone();
            handles.push(std::thread::spawn(move || {
                let name = format!("coll_{}", i);
                cache.insert(&name, "id".to_string(), i as u64);
                assert!(cache.get(&name).is_some());
                cache.get_alias(&name);
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(cache.len(), 10);
    }
}
