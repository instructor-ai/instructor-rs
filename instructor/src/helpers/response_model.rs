use std::collections::HashMap;

use instruct_macros_types::{Parameter, ParameterInfo, StructInfo};
use openai_api_rs::v1::chat_completion::{self, JSONSchemaDefine};

fn get_required_properties(info: &StructInfo) -> Vec<String> {
    let mut required = Vec::new();

    for param in info.parameters.iter() {
        match param {
            Parameter::Field(field_info) => {
                if !field_info.is_optional {
                    required.push(field_info.name.clone());
                }
            }
            Parameter::Struct(struct_info) => {
                if !struct_info.is_optional {
                    required.push(struct_info.name.clone());
                }
            }
            Parameter::Enum(enum_info) => {
                if !enum_info.is_optional {
                    required.push(enum_info.title.clone());
                }
            }
        }
    }
    required
}

fn convert_parameter_type(info: &str) -> chat_completion::JSONSchemaType {
    match info {
        "String" | "char" => chat_completion::JSONSchemaType::String,
        "u8" | "i8" | "u16" | "i16" | "u32" | "i32" | "u64" | "i64" | "u128" | "i128" | "usize"
        | "isize" => chat_completion::JSONSchemaType::Number,
        "bool" => chat_completion::JSONSchemaType::Boolean,
        _ => panic!("Unsupported type: {}", info),
    }
}

fn get_base_type(field_info: &ParameterInfo) -> &str {
    if field_info.r#type.starts_with("Option<") && field_info.r#type.ends_with('>') {
        &field_info.r#type[7..field_info.r#type.len() - 1]
    } else {
        &field_info.r#type
    }
}

fn get_response_model_parameters(t: &StructInfo) -> HashMap<String, Box<JSONSchemaDefine>> {
    let mut properties = HashMap::new();

    for param in t.parameters.iter() {
        match param {
            Parameter::Field(field_info) => {
                let parameter_name = field_info.name.clone();
                let parameter_description = field_info.comment.clone();

                let base_type = get_base_type(field_info);
                let parameter_type = if field_info.is_list {
                    chat_completion::JSONSchemaType::Array
                } else {
                    convert_parameter_type(&base_type.to_string())
                };

                let items = if field_info.is_list {
                    Some(Box::new(chat_completion::JSONSchemaDefine {
                        schema_type: Some(convert_parameter_type(base_type)),
                        ..Default::default()
                    }))
                } else {
                    None
                };

                properties.insert(
                    parameter_name,
                    Box::new(chat_completion::JSONSchemaDefine {
                        schema_type: Some(parameter_type),
                        description: Some(parameter_description),
                        items: items,
                        ..Default::default()
                    }),
                );
            }
            Parameter::Enum(enum_info) => {
                let parameter_name = enum_info.title.clone();
                let parameter_description = enum_info.description.clone();
                let enum_values: Vec<String> = enum_info.r#enum.iter().map(|e| e.clone()).collect();

                let parameter_type = if enum_info.is_list {
                    chat_completion::JSONSchemaType::Array
                } else {
                    chat_completion::JSONSchemaType::String
                };

                let items = if enum_info.is_list {
                    Some(Box::new(chat_completion::JSONSchemaDefine {
                        schema_type: Some(chat_completion::JSONSchemaType::String),
                        enum_values: Some(enum_values.clone()),
                        ..Default::default()
                    }))
                } else {
                    None
                };

                properties.insert(
                    parameter_name,
                    Box::new(chat_completion::JSONSchemaDefine {
                        schema_type: Some(parameter_type),
                        description: Some(parameter_description),
                        items: items,
                        enum_values: if enum_info.is_list {
                            None
                        } else {
                            Some(enum_values.clone()) // Clone here to avoid move error
                        },
                        ..Default::default()
                    }),
                );
            }
            Parameter::Struct(struct_info) => {
                let parameter_name = struct_info.name.clone();
                let parameter_description = struct_info.description.clone();

                let parameter_type = if struct_info.is_list {
                    chat_completion::JSONSchemaType::Array
                } else {
                    chat_completion::JSONSchemaType::Object
                };

                let items = if struct_info.is_list {
                    Some(Box::new(chat_completion::JSONSchemaDefine {
                        schema_type: Some(chat_completion::JSONSchemaType::Object),
                        properties: Some(get_response_model_parameters(struct_info)),
                        ..Default::default()
                    }))
                } else {
                    None
                };

                let struct_properties = if struct_info.is_list {
                    None
                } else {
                    Some(get_response_model_parameters(struct_info))
                };

                properties.insert(
                    parameter_name,
                    Box::new(chat_completion::JSONSchemaDefine {
                        schema_type: Some(parameter_type),
                        description: Some(parameter_description),
                        items,
                        properties: struct_properties,
                        ..Default::default()
                    }),
                );
            }
        }
    }

    properties
}

pub fn get_response_model(t: StructInfo) -> chat_completion::FunctionParameters {
    // // TODO: Fix this up
    let properties = get_response_model_parameters(&t);
    let required_fields = get_required_properties(&t);

    chat_completion::FunctionParameters {
        schema_type: chat_completion::JSONSchemaType::Object,
        properties: Some(properties),
        required: Some(required_fields),
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use instruct_macros::InstructMacro;
    use instruct_macros_types::{
        InstructMacro, InstructMacroResult, Parameter, ParameterInfo, StructInfo,
    };
    use serde::{Deserialize, Serialize};

    #[test]
    fn test_person_with_nested_address() {
        #[derive(InstructMacro, Debug, Serialize, Deserialize)]
        struct Person {
            #[description("The name of the person")]
            name: String,
            #[description("The age of the person")]
            age: u8,
            #[description("The address of the person")]
            address: Address,
        }

        #[derive(InstructMacro, Debug, Serialize, Deserialize)]
        #[description("The address of the person")]
        struct Address {
            #[description("The street of the address")]
            street: String,
            #[description("The city of the address")]
            city: String,
            #[description("The zip code of the address")]
            zip_code: String,
        }

        let struct_info = Person::get_info();
        let parsed_model: StructInfo = match struct_info {
            InstructMacroResult::Struct(info) => info,
            _ => {
                panic!("Expected StructInfo but got a different InstructMacroResult variant");
            }
        };
        let parameters = get_response_model(parsed_model);

        let expected_parameters = chat_completion::FunctionParameters {
            schema_type: chat_completion::JSONSchemaType::Object,
            properties: Some({
                let mut props = std::collections::HashMap::new();
                props.insert(
                    "name".to_string(),
                    Box::new(chat_completion::JSONSchemaDefine {
                        schema_type: Some(chat_completion::JSONSchemaType::String),
                        description: Some("The name of the person".to_string()),
                        ..Default::default()
                    }),
                );
                props.insert(
                    "age".to_string(),
                    Box::new(chat_completion::JSONSchemaDefine {
                        schema_type: Some(chat_completion::JSONSchemaType::Number),
                        description: Some("The age of the person".to_string()),
                        ..Default::default()
                    }),
                );
                props.insert(
                    "address".to_string(),
                    Box::new(chat_completion::JSONSchemaDefine {
                        schema_type: Some(chat_completion::JSONSchemaType::Object),
                        description: Some("The address of the person".to_string()),
                        properties: Some({
                            let mut address_props = std::collections::HashMap::new();
                            address_props.insert(
                                "street".to_string(),
                                Box::new(chat_completion::JSONSchemaDefine {
                                    schema_type: Some(chat_completion::JSONSchemaType::String),
                                    description: Some("The street of the address".to_string()),
                                    ..Default::default()
                                }),
                            );
                            address_props.insert(
                                "city".to_string(),
                                Box::new(chat_completion::JSONSchemaDefine {
                                    schema_type: Some(chat_completion::JSONSchemaType::String),
                                    description: Some("The city of the address".to_string()),
                                    ..Default::default()
                                }),
                            );
                            address_props.insert(
                                "zip_code".to_string(),
                                Box::new(chat_completion::JSONSchemaDefine {
                                    schema_type: Some(chat_completion::JSONSchemaType::String),
                                    description: Some("The zip code of the address".to_string()),
                                    ..Default::default()
                                }),
                            );
                            address_props
                        }),
                        ..Default::default()
                    }),
                );
                props
            }),
            required: Some(vec![
                "name".to_string(),
                "age".to_string(),
                "address".to_string(),
            ]),
        };

        assert_eq!(parameters, expected_parameters);
    }

    #[test]
    fn test_person_with_enum_job() {
        #[derive(InstructMacro, Debug, Serialize, Deserialize)]
        struct Person {
            #[description("The name of the person")]
            name: String,
            #[description("The age of the person")]
            age: u8,
            job: Job,
        }

        #[derive(InstructMacro, Debug, Serialize, Deserialize)]
        #[description("The job of the person")]
        enum Job {
            #[description("Software Developer")]
            Developer,
            #[description("Teacher")]
            Teacher,
            #[description("Artist")]
            Artist,
        }

        let struct_info = Person::get_info();
        let parsed_model: StructInfo = match struct_info {
            InstructMacroResult::Struct(info) => info,
            _ => {
                panic!("Expected StructInfo but got a different InstructMacroResult variant");
            }
        };
        let parameters = get_response_model(parsed_model);

        let expected_parameters = chat_completion::FunctionParameters {
            schema_type: chat_completion::JSONSchemaType::Object,
            properties: Some({
                let mut props = std::collections::HashMap::new();
                props.insert(
                    "name".to_string(),
                    Box::new(chat_completion::JSONSchemaDefine {
                        schema_type: Some(chat_completion::JSONSchemaType::String),
                        description: Some("The name of the person".to_string()),
                        ..Default::default()
                    }),
                );
                props.insert(
                    "age".to_string(),
                    Box::new(chat_completion::JSONSchemaDefine {
                        schema_type: Some(chat_completion::JSONSchemaType::Number),
                        description: Some("The age of the person".to_string()),
                        ..Default::default()
                    }),
                );
                props.insert(
                    "job".to_string(),
                    Box::new(chat_completion::JSONSchemaDefine {
                        schema_type: Some(chat_completion::JSONSchemaType::String),
                        description: Some("The job of the person".to_string()),
                        enum_values: Some(vec![
                            "Developer".to_string(),
                            "Teacher".to_string(),
                            "Artist".to_string(),
                        ]),
                        ..Default::default()
                    }),
                );
                props
            }),
            required: Some(vec![
                "name".to_string(),
                "age".to_string(),
                "job".to_string(),
            ]),
        };

        assert_eq!(expected_parameters, parameters);
    }

    #[test]
    fn test_simple_struct() {
        #[derive(InstructMacro, Debug, Serialize, Deserialize)]
        struct SimpleStruct {
            #[description("The name of the user")]
            name: String,
            #[description("The age of the user")]
            age: u8,
        }

        let struct_info = SimpleStruct::get_info();
        let parsed_model: StructInfo = match struct_info {
            InstructMacroResult::Struct(info) => info,
            _ => {
                panic!("Expected StructInfo but got a different InstructMacroResult variant");
            }
        };
        let parameters = get_response_model(parsed_model);

        let expected_parameters = chat_completion::FunctionParameters {
            schema_type: chat_completion::JSONSchemaType::Object,
            properties: Some({
                let mut props = std::collections::HashMap::new();
                props.insert(
                    "name".to_string(),
                    Box::new(chat_completion::JSONSchemaDefine {
                        schema_type: Some(chat_completion::JSONSchemaType::String),
                        description: Some("The name of the user".to_string()),
                        ..Default::default()
                    }),
                );
                props.insert(
                    "age".to_string(),
                    Box::new(chat_completion::JSONSchemaDefine {
                        schema_type: Some(chat_completion::JSONSchemaType::Number),
                        description: Some("The age of the user".to_string()),
                        ..Default::default()
                    }),
                );
                props
            }),
            required: Some(vec!["name".to_string(), "age".to_string()]),
        };

        assert_eq!(expected_parameters, parameters);
    }

    #[test]
    fn test_struct_with_optional_field() {
        #[derive(InstructMacro, Debug, Serialize, Deserialize)]
        struct StructWithOptionalField {
            #[description("The name of the user")]
            name: String,
            #[description("The age of the user")]
            age: Option<u8>,
        }

        let struct_info = StructWithOptionalField::get_info();
        let parsed_model: StructInfo = match struct_info {
            InstructMacroResult::Struct(info) => info,
            _ => {
                panic!("Expected StructInfo but got a different InstructMacroResult variant");
            }
        };
        let parameters = get_response_model(parsed_model);

        let expected_parameters = chat_completion::FunctionParameters {
            schema_type: chat_completion::JSONSchemaType::Object,
            properties: Some({
                let mut props = std::collections::HashMap::new();
                props.insert(
                    "name".to_string(),
                    Box::new(chat_completion::JSONSchemaDefine {
                        schema_type: Some(chat_completion::JSONSchemaType::String),
                        description: Some("The name of the user".to_string()),
                        ..Default::default()
                    }),
                );
                props.insert(
                    "age".to_string(),
                    Box::new(chat_completion::JSONSchemaDefine {
                        schema_type: Some(chat_completion::JSONSchemaType::Number),
                        description: Some("The age of the user".to_string()),
                        ..Default::default()
                    }),
                );
                props
            }),
            required: Some(vec!["name".to_string()]), // Only "name" should be required
        };

        assert_eq!(expected_parameters, parameters);
    }

    #[test]
    fn test_struct_with_nested_optional_field() {
        #[derive(InstructMacro, Debug, Serialize, Deserialize)]
        struct User {
            name: String,
            age: u8,
        }

        #[derive(InstructMacro, Debug, Serialize, Deserialize)]
        struct MaybeUser {
            user: Option<User>,
        }

        let struct_info = MaybeUser::get_info();
        let parsed_model: StructInfo = match struct_info {
            InstructMacroResult::Struct(info) => info,
            _ => {
                panic!("Expected StructInfo but got a different InstructMacroResult variant");
            }
        };
        let parameters = get_response_model(parsed_model);

        let expected_parameters = chat_completion::FunctionParameters {
            schema_type: chat_completion::JSONSchemaType::Object,
            properties: Some({
                let mut props = std::collections::HashMap::new();
                props.insert(
                    "user".to_string(),
                    Box::new(chat_completion::JSONSchemaDefine {
                        schema_type: Some(chat_completion::JSONSchemaType::Object),
                        description: Some("".to_string()),
                        properties: Some({
                            let mut user_props = std::collections::HashMap::new();
                            user_props.insert(
                                "name".to_string(),
                                Box::new(chat_completion::JSONSchemaDefine {
                                    schema_type: Some(chat_completion::JSONSchemaType::String),
                                    description: Some("".to_string()),
                                    ..Default::default()
                                }),
                            );
                            user_props.insert(
                                "age".to_string(),
                                Box::new(chat_completion::JSONSchemaDefine {
                                    schema_type: Some(chat_completion::JSONSchemaType::Number),
                                    description: Some("".to_string()),
                                    ..Default::default()
                                }),
                            );
                            user_props
                        }),
                        ..Default::default()
                    }),
                );
                props
            }),
            required: Some(vec![]), // No required fields
        };

        assert_eq!(expected_parameters, parameters);
    }

    #[test]
    fn test_struct_with_vec_of_i32() {
        #[derive(InstructMacro, Debug, Serialize, Deserialize)]
        struct Numbers {
            #[description("A list of numbers")]
            numbers: Vec<i32>,
        }

        let struct_info = Numbers::get_info();
        let parsed_model: StructInfo = match struct_info {
            InstructMacroResult::Struct(info) => info,
            _ => {
                panic!("Expected StructInfo but got a different InstructMacroResult variant");
            }
        };
        let parameters = get_response_model(parsed_model);

        let expected_parameters = chat_completion::FunctionParameters {
            schema_type: chat_completion::JSONSchemaType::Object,
            properties: Some({
                let mut props = std::collections::HashMap::new();
                props.insert(
                    "numbers".to_string(),
                    Box::new(chat_completion::JSONSchemaDefine {
                        schema_type: Some(chat_completion::JSONSchemaType::Array),
                        description: Some("A list of numbers".to_string()),
                        items: Some(Box::new(chat_completion::JSONSchemaDefine {
                            schema_type: Some(chat_completion::JSONSchemaType::Number),
                            ..Default::default()
                        })),
                        ..Default::default()
                    }),
                );
                props
            }),
            required: Some(vec!["numbers".to_string()]), // "numbers" is a required field
        };

        assert_eq!(expected_parameters, parameters);
    }

    #[test]
    fn test_struct_with_vec_of_users() {
        #[derive(InstructMacro, Debug, Serialize, Deserialize)]
        struct User {
            name: String,
        }
        #[derive(InstructMacro, Debug, Serialize, Deserialize)]
        struct Users {
            #[description("A list of users")]
            users: Vec<User>,
        }

        let struct_info = Users::get_info();
        let parsed_model: StructInfo = match struct_info {
            InstructMacroResult::Struct(info) => info,
            _ => {
                panic!("Expected StructInfo but got a different InstructMacroResult variant");
            }
        };
        let parameters = get_response_model(parsed_model);

        let expected_parameters = chat_completion::FunctionParameters {
            schema_type: chat_completion::JSONSchemaType::Object,
            properties: Some({
                let mut props = std::collections::HashMap::new();
                props.insert(
                    "users".to_string(),
                    Box::new(chat_completion::JSONSchemaDefine {
                        schema_type: Some(chat_completion::JSONSchemaType::Array),
                        description: Some("A list of users".to_string()),
                        required: None,
                        items: Some(Box::new(chat_completion::JSONSchemaDefine {
                            schema_type: Some(chat_completion::JSONSchemaType::Object),
                            properties: Some({
                                let mut user_props = std::collections::HashMap::new();
                                user_props.insert(
                                    "name".to_string(),
                                    Box::new(chat_completion::JSONSchemaDefine {
                                        schema_type: Some(chat_completion::JSONSchemaType::String),
                                        description: Some("".to_string()),
                                        ..Default::default()
                                    }),
                                );
                                user_props
                            }),
                            ..Default::default()
                        })),
                        ..Default::default()
                    }),
                );
                props
            }),
            required: Some(vec!["users".to_string()]), // "users" is a required field
        };
        assert_eq!(expected_parameters, parameters);
    }

    #[test]
    fn test_struct_with_vec_of_enums() {
        #[derive(InstructMacro, Debug, Serialize, Deserialize)]
        enum Job {
            Developer,
            Teacher,
            Artist,
        }

        #[derive(InstructMacro, Debug, Serialize, Deserialize)]
        struct Person {
            #[description("The name of the person")]
            name: String,
            #[description("The jobs of the person")]
            jobs: Vec<Job>,
        }

        let struct_info = Person::get_info();
        let parsed_model: StructInfo = match struct_info {
            InstructMacroResult::Struct(info) => info,
            _ => {
                panic!("Expected StructInfo but got a different InstructMacroResult variant");
            }
        };
        let parameters = get_response_model(parsed_model);

        let expected_parameters = chat_completion::FunctionParameters {
            schema_type: chat_completion::JSONSchemaType::Object,
            properties: Some({
                let mut props = std::collections::HashMap::new();
                props.insert(
                    "name".to_string(),
                    Box::new(chat_completion::JSONSchemaDefine {
                        schema_type: Some(chat_completion::JSONSchemaType::String),
                        description: Some("The name of the person".to_string()),
                        ..Default::default()
                    }),
                );
                props.insert(
                    "jobs".to_string(),
                    Box::new(chat_completion::JSONSchemaDefine {
                        schema_type: Some(chat_completion::JSONSchemaType::Array),
                        description: Some("The jobs of the person".to_string()),
                        items: Some(Box::new(chat_completion::JSONSchemaDefine {
                            schema_type: Some(chat_completion::JSONSchemaType::String),
                            enum_values: Some(vec![
                                "Developer".to_string(),
                                "Teacher".to_string(),
                                "Artist".to_string(),
                            ]),
                            ..Default::default()
                        })),
                        ..Default::default()
                    }),
                );
                props
            }),
            required: Some(vec!["name".to_string(), "jobs".to_string()]),
        };

        assert_eq!(expected_parameters, parameters);
    }
}
