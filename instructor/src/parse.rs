use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StructInfo {
    pub name: String,
    pub description: String,
    pub parameters: Vec<ParameterInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParameterInfo {
    pub name: String,
    pub r#type: String,
    pub comment: String,
}
