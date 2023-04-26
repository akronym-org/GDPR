use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Deserialize, Serialize)]
pub enum FilterOperator {
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
    In,
    Nin,
    Null,
    Nnull,
    Contains,
    Ncontains,
    Icontains,
    Between,
    Nbetween,
    Empty,
    Nempty,
    Intersects,
    Nintersects,
    IntersectsBbox,
    NintersectsBbox,
}

#[derive(Deserialize, Serialize)]
pub enum ClientFilterOperator {
    FilterOperator,
    StartsWith,
    NstartsWith,
    EndsWith,
    NendsWith,
    Regex,
}

#[derive(Deserialize, Serialize)]
pub enum Filter {
    LogicalFilter,
    FieldFilter,
}

#[derive(Deserialize, Serialize)]
pub struct LogicalFilterOR {
    _or: Vec<Filter>,
}

#[derive(Deserialize, Serialize)]
pub struct LogicalFilterAND {
    _and: Vec<Filter>,
}

#[derive(Deserialize, Serialize)]
pub enum LogicalFilter {
    LogicalFilterOR,
    LogicalFilterAND,
}

#[derive(Deserialize, Serialize)]
pub struct FieldFilter {
    field: String,
    filter: FieldFilterOperator,
}

#[derive(Deserialize, Serialize)]
pub enum StringNumberBool {
    String,
    F64,
    Bool,
}

#[derive(Deserialize, Serialize)]
pub enum StringNumber {
    String,
    F64,
}

#[derive(Deserialize, Serialize)]
pub struct FieldFilterOperator {
    _eq: Option<StringNumberBool>,
    _neq: Option<StringNumberBool>,
    _lt: Option<StringNumber>,
    _lte: Option<StringNumber>,
    _gt: Option<StringNumber>,
    _gte: Option<StringNumber>,
    _in: Option<Vec<StringNumber>>,
    _nin: Option<Vec<StringNumber>>,
    _null: Option<bool>,
    _nnull: Option<bool>,
    _contains: Option<String>,
    _ncontains: Option<String>,
    _icontains: Option<String>,
    _starts_with: Option<String>,
    _nstarts_with: Option<String>,
    _ends_with: Option<String>,
    _nends_with: Option<String>,
    _between: Option<Vec<StringNumber>>,
    _nbetween: Option<Vec<StringNumber>>,
    _empty: Option<bool>,
    _nempty: Option<bool>,
    _intersects: Option<String>,
    _nintersects: Option<String>,
    _intersects_bbox: Option<String>,
    _nintersects_bbox: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct FieldValidationOperator {
    _submitted: Option<bool>,
    _regex: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub enum PermissionsAction {
    Create,
    Read,
    Update,
    Delete,
    Comment,
    Explain,
    Share,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Permission {
    pub id: i32,
    pub role: Option<Uuid>,
    pub collection: String,
    pub action: String,
    pub permissions: Option<serde_json::Value>,
    pub validation: Option<serde_json::Value>,
    pub presets: Option<serde_json::Value>,
    pub fields: Option<String>,
    // pub system: Option<bool>,
}
