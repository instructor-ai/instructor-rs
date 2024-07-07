use serde::{Deserialize, Serialize};

pub trait InstructMacro {
    fn get_info() -> StructInfo;
    fn validate(&self) -> Result<(), String>;
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StructInfo {
    pub name: String,
    pub description: String,
    pub parameters: Vec<Parameter>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Parameter {
    Struct(StructInfo),
    Field(ParameterInfo),
    Enum(EnumInfo),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ParameterInfo {
    pub name: String,
    pub r#type: String,
    pub comment: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EnumInfo {
    pub title: String,
    pub r#enum: Vec<String>,
    pub r#type: String,
    pub description: String,
}

pub struct FieldInfo {
    pub name: String,
    pub description: String,
    pub r#type: String,
    pub is_complex: bool,
}
