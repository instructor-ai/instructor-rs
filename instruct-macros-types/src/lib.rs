use serde::{Deserialize, Serialize};

pub trait InstructMacro {
    fn get_info() -> StructInfo;
    fn validate(&self) -> Result<(), String>;
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StructInfo {
    pub name: String,
    pub description: String,
    pub parameters: Vec<ParameterInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ParameterInfo {
    pub name: String,
    pub r#type: String,
    pub comment: String,
}

pub struct FieldInfo {
    pub name: String,
    pub description: String,
    pub r#type: String,
    pub is_complex: bool,
}
