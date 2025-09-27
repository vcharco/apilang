#[derive(Debug, Clone)]
pub enum FieldType {
    String,
    Number,
    Boolean,
    List(Box<FieldType>),
    Custom(String), // For types like uuid, int, etc.
}

#[derive(Debug, Clone)]
pub enum Constraint {
    Min(f64),
    Max(f64),
    Email,
    Phone,
    Regex(String),
}

#[derive(Debug, Clone)]
pub enum Visibility {
    Public,
    Private(Vec<String>), // For roles
}

#[derive(Debug, Clone)]
pub enum CacheConfig {
    Cached(String),
    Evict(String),
}

#[derive(Debug, Clone)]
pub enum HttpMethod {
    Get,
    Post,
    Patch,
    Put,
    Delete,
    Options,
    Head,
}
