use super::expressions::*;

#[derive(Debug, Clone)]
pub enum TopLevel {
    Model(ModelDef),
    Configure(ConfigureDef),
    Endpoint(EndpointDef),
}

#[derive(Debug, Clone)]
pub struct ModelDef {
    pub name: String,
    pub fields: Vec<FieldDef>,
}

#[derive(Debug, Clone)]
pub struct FieldDef {
    pub name: String,
    pub field_type: FieldType,
    pub constraints: Vec<Constraint>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ConfigureDef {
    pub settings: Vec<ConfigSetting>,
}

#[derive(Debug, Clone)]
pub enum ConfigSetting {
    BasePath(String),
    Throttle(f64),
    Private(Vec<String>),
}

#[derive(Debug, Clone)]
pub struct EndpointDef {
    pub visibility: Visibility,
    pub cache_config: Option<CacheConfig>,
    pub method: HttpMethod,
    pub path: String,
    pub params: Option<ParamsDef>,
    pub query: Option<QueryDef>,
    pub body: Option<BodyDef>,
    pub call: String,
}

#[derive(Debug, Clone)]
pub struct ParamsDef {
    pub name: String,
    pub param_type: String,
}

#[derive(Debug, Clone)]
pub struct QueryDef {
    pub query_type: String,
}

#[derive(Debug, Clone)]
pub struct BodyDef {
    pub body_type: String,
    pub tags: Option<Vec<String>>,
}
