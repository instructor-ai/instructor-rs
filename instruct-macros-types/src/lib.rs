use serde::{Deserialize, Serialize};

pub trait InstructMacro {
    fn get_info() -> InstructMacroResult;
    fn validate(&self) -> Result<(), String>;
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum InstructMacroResult {
    Struct(StructInfo),
    Enum(EnumInfo),
}

impl InstructMacroResult {
    pub fn wrap_info(self, new_name: String) -> Parameter {
        match self {
            InstructMacroResult::Struct(struct_info) => struct_info.wrap_info(new_name),
            InstructMacroResult::Enum(enum_info) => enum_info.wrap_info(new_name),
        }
    }

    pub fn override_description(self, new_description: String) -> InstructMacroResult {
        match self {
            InstructMacroResult::Struct(struct_info) => {
                InstructMacroResult::Struct(struct_info.override_description(new_description))
            }
            InstructMacroResult::Enum(enum_info) => {
                InstructMacroResult::Enum(enum_info.override_description(new_description))
            }
        }
    }

    pub fn set_optional(self, is_optional: bool) -> InstructMacroResult {
        match self {
            InstructMacroResult::Struct(struct_info) => {
                InstructMacroResult::Struct(struct_info.set_optional(is_optional))
            }
            InstructMacroResult::Enum(enum_info) => {
                InstructMacroResult::Enum(enum_info.set_optional(is_optional))
            }
        }
    }

    pub fn set_list(self, is_list: bool) -> InstructMacroResult {
        match self {
            InstructMacroResult::Struct(struct_info) => {
                InstructMacroResult::Struct(struct_info.set_list(is_list))
            }
            InstructMacroResult::Enum(enum_info) => {
                InstructMacroResult::Enum(enum_info.set_list(is_list))
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StructInfo {
    pub name: String,
    pub description: String,
    pub parameters: Vec<Parameter>,
    pub is_optional: bool,
    pub is_list: bool,
}

impl StructInfo {
    pub fn wrap_info(mut self, new_name: String) -> Parameter {
        self.name = new_name;
        Parameter::Struct(self)
    }

    pub fn override_description(mut self, new_description: String) -> StructInfo {
        if new_description.len() > 0 {
            self.description = new_description;
        }
        self
    }

    pub fn set_optional(mut self, is_optional: bool) -> StructInfo {
        self.is_optional = is_optional;
        self
    }

    pub fn set_list(mut self, is_list: bool) -> StructInfo {
        self.is_list = is_list;
        self
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
    pub is_optional: bool,
    pub is_list: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EnumInfo {
    pub title: String,
    pub r#enum: Vec<String>,
    pub r#type: String,
    pub description: String,
    pub is_optional: bool,
    pub is_list: bool,
}

impl EnumInfo {
    pub fn wrap_info(mut self, new_name: String) -> Parameter {
        self.title = new_name;
        Parameter::Enum(self)
    }

    pub fn override_description(mut self, new_description: String) -> EnumInfo {
        if new_description.len() > 0 {
            self.description = new_description;
        }
        self
    }

    pub fn set_optional(mut self, is_optional: bool) -> EnumInfo {
        self.is_optional = is_optional;
        self
    }

    pub fn set_list(mut self, is_list: bool) -> EnumInfo {
        self.is_list = is_list;
        self
    }
}

pub struct FieldInfo {
    pub name: String,
    pub description: String,
    pub r#type: String,
    pub is_complex: bool,
    pub is_optional: bool,
    pub is_list: bool,
}
