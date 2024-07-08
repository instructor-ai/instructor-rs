extern crate instruct_macros_types;

use instruct_macros::InstructMacro;
use instruct_macros_types::{
    InstructMacro, InstructMacroResult, Parameter, ParameterInfo, StructInfo,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_type_support() {
        #[derive(InstructMacro, Debug)]
        #[allow(dead_code)]
        #[description("This is a struct with Option types")]
        struct TestOptionStruct {
            #[description("This is an optional string field")]
            pub field1: Option<String>,
            #[description("This is an optional integer field")]
            pub field2: Option<i32>,
        }

        let info = TestOptionStruct::get_info();
        let desired_struct = StructInfo {
            name: "TestOptionStruct".to_string(),
            description: "This is a struct with Option types".to_string(),
            is_optional: false,
            is_list: false,
            parameters: vec![
                Parameter::Field(ParameterInfo {
                    name: "field1".to_string(),
                    r#type: "Option<String>".to_string(),
                    comment: "This is an optional string field".to_string(),
                    is_optional: true,
                    is_list: false,
                }),
                Parameter::Field(ParameterInfo {
                    name: "field2".to_string(),
                    r#type: "Option<i32>".to_string(),
                    comment: "This is an optional integer field".to_string(),
                    is_optional: true,
                    is_list: false,
                }),
            ],
        };

        let info_struct = match info {
            InstructMacroResult::Struct(s) => s,
            _ => panic!("Expected StructInfo"),
        };

        assert_eq!(info_struct, desired_struct);
    }

    #[test]
    fn test_option_maybe_struct() {
        #[derive(InstructMacro, Debug)]
        #[allow(dead_code)]
        #[description("This is a user struct")]
        struct User {
            #[description("This is the user's name")]
            pub name: String,
            #[description("This is the user's age")]
            pub age: i32,
        }

        #[derive(InstructMacro, Debug)]
        #[allow(dead_code)]
        #[description("This is a struct with Option<user> type")]
        struct MaybeUser {
            #[description("This is an optional user field")]
            pub user: Option<User>,
        }

        let info = MaybeUser::get_info();
        let desired_struct = StructInfo {
            name: "MaybeUser".to_string(),
            description: "This is a struct with Option<user> type".to_string(),
            parameters: vec![Parameter::Struct(StructInfo {
                name: "user".to_string(),
                description: "This is an optional user field".to_string(),
                parameters: vec![
                    Parameter::Field(ParameterInfo {
                        name: "name".to_string(),
                        r#type: "String".to_string(),
                        comment: "This is the user's name".to_string(),
                        is_optional: false,
                        is_list: false,
                    }),
                    Parameter::Field(ParameterInfo {
                        name: "age".to_string(),
                        r#type: "i32".to_string(),
                        comment: "This is the user's age".to_string(),
                        is_optional: false,
                        is_list: false,
                    }),
                ],
                is_optional: true,
                is_list: false,
            })],
            is_optional: false,
            is_list: false,
        };

        let info_struct = match info {
            InstructMacroResult::Struct(s) => s,
            _ => panic!("Expected StructInfo"),
        };

        assert_eq!(info_struct, desired_struct);
    }
}
