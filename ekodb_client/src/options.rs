//! Options structs for CRUD operations
//!
//! All options follow the builder pattern for ergonomic usage.

/// Options for insert operations
#[derive(Debug, Clone, Default)]
pub struct InsertOptions {
    pub ttl: Option<String>,
    pub bypass_ripple: Option<bool>,
    pub transaction_id: Option<String>,
    pub bypass_cache: Option<bool>,
}

impl InsertOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ttl(mut self, ttl: impl Into<String>) -> Self {
        self.ttl = Some(ttl.into());
        self
    }

    pub fn bypass_ripple(mut self, bypass: bool) -> Self {
        self.bypass_ripple = Some(bypass);
        self
    }

    pub fn transaction_id(mut self, id: impl Into<String>) -> Self {
        self.transaction_id = Some(id.into());
        self
    }

    pub fn bypass_cache(mut self, bypass: bool) -> Self {
        self.bypass_cache = Some(bypass);
        self
    }
}

/// Options for update operations
#[derive(Debug, Clone, Default)]
pub struct UpdateOptions {
    pub bypass_ripple: Option<bool>,
    pub transaction_id: Option<String>,
    pub bypass_cache: Option<bool>,
    pub select_fields: Option<Vec<String>>,
    pub exclude_fields: Option<Vec<String>>,
}

impl UpdateOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bypass_ripple(mut self, bypass: bool) -> Self {
        self.bypass_ripple = Some(bypass);
        self
    }

    pub fn transaction_id(mut self, id: impl Into<String>) -> Self {
        self.transaction_id = Some(id.into());
        self
    }

    pub fn bypass_cache(mut self, bypass: bool) -> Self {
        self.bypass_cache = Some(bypass);
        self
    }

    pub fn select_fields(mut self, fields: Vec<String>) -> Self {
        self.select_fields = Some(fields);
        self
    }

    pub fn exclude_fields(mut self, fields: Vec<String>) -> Self {
        self.exclude_fields = Some(fields);
        self
    }
}

/// Options for delete operations
#[derive(Debug, Clone, Default)]
pub struct DeleteOptions {
    pub bypass_ripple: Option<bool>,
    pub transaction_id: Option<String>,
}

impl DeleteOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bypass_ripple(mut self, bypass: bool) -> Self {
        self.bypass_ripple = Some(bypass);
        self
    }

    pub fn transaction_id(mut self, id: impl Into<String>) -> Self {
        self.transaction_id = Some(id.into());
        self
    }
}

/// Options for upsert operations
#[derive(Debug, Clone, Default)]
pub struct UpsertOptions {
    pub ttl: Option<String>,
    pub bypass_ripple: Option<bool>,
    pub transaction_id: Option<String>,
    pub bypass_cache: Option<bool>,
}

impl UpsertOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ttl(mut self, ttl: impl Into<String>) -> Self {
        self.ttl = Some(ttl.into());
        self
    }

    pub fn bypass_ripple(mut self, bypass: bool) -> Self {
        self.bypass_ripple = Some(bypass);
        self
    }

    pub fn transaction_id(mut self, id: impl Into<String>) -> Self {
        self.transaction_id = Some(id.into());
        self
    }

    pub fn bypass_cache(mut self, bypass: bool) -> Self {
        self.bypass_cache = Some(bypass);
        self
    }
}

/// Options for batch insert operations
#[derive(Debug, Clone, Default)]
pub struct BatchInsertOptions {
    pub bypass_ripple: Option<bool>,
    pub transaction_id: Option<String>,
}

impl BatchInsertOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bypass_ripple(mut self, bypass: bool) -> Self {
        self.bypass_ripple = Some(bypass);
        self
    }

    pub fn transaction_id(mut self, id: impl Into<String>) -> Self {
        self.transaction_id = Some(id.into());
        self
    }
}

/// Options for batch update operations
#[derive(Debug, Clone, Default)]
pub struct BatchUpdateOptions {
    pub bypass_ripple: Option<bool>,
    pub transaction_id: Option<String>,
}

impl BatchUpdateOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bypass_ripple(mut self, bypass: bool) -> Self {
        self.bypass_ripple = Some(bypass);
        self
    }

    pub fn transaction_id(mut self, id: impl Into<String>) -> Self {
        self.transaction_id = Some(id.into());
        self
    }
}

/// Options for batch delete operations
#[derive(Debug, Clone, Default)]
pub struct BatchDeleteOptions {
    pub bypass_ripple: Option<bool>,
    pub transaction_id: Option<String>,
}

impl BatchDeleteOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bypass_ripple(mut self, bypass: bool) -> Self {
        self.bypass_ripple = Some(bypass);
        self
    }

    pub fn transaction_id(mut self, id: impl Into<String>) -> Self {
        self.transaction_id = Some(id.into());
        self
    }
}
