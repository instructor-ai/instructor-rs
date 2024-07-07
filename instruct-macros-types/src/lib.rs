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

impl StructInfo {
    pub fn wrap_info(mut self, new_name: String) -> Parameter {
        self.name = new_name;
        Parameter::Struct(self)
    }
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

impl EnumInfo {
    pub fn wrap_info(mut self, new_name: String) -> Parameter {
        self.title = new_name;
        Parameter::Enum(self)
    }
}

pub struct FieldInfo {
    pub name: String,
    pub description: String,
    pub r#type: String,
    pub is_complex: bool,
}
