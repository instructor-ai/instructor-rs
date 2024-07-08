extern crate instruct_macros_types;

use instruct_macros::InstructMacro;
use instruct_macros_types::{
    InstructMacro, InstructMacroResult, Parameter, ParameterInfo, StructInfo,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_simple_type() {
        #[derive(InstructMacro, Debug)]
        #[description("This is a struct with Option types")]
        struct Numbers {
            #[description("This is a list of numbers")]
            pub numbers: Vec<i32>,
        }

        let info = Numbers::get_info();

        let desired_struct = StructInfo {
            name: "Numbers".to_string(),
            description: "This is a struct with Option types".to_string(),
            parameters: vec![Parameter::Field(ParameterInfo {
                name: "numbers".to_string(),
                r#type: "i32".to_string(),
                comment: "This is a list of numbers".to_string(),
                is_optional: false,
                is_list: true,
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

    #[test]
    fn test_vec_struct() {
        #[derive(InstructMacro, Debug)]
        #[description("This is a struct with Option types")]
        struct User {
            #[description("This is a list of numbers")]
            pub name: String,
        }

        #[derive(InstructMacro, Debug)]
        #[description("This is a struct with Option types")]
        struct Users {
            #[description("This is a list of users")]
            pub users: Vec<User>,
        }

        let info = Users::get_info();

        let desired_struct = StructInfo {
            name: "Users".to_string(),
            description: "This is a struct with Option types".to_string(),
            parameters: vec![Parameter::Struct(StructInfo {
                name: "users".to_string(),
                description: "This is a list of users".to_string(),
                parameters: vec![Parameter::Field(ParameterInfo {
                    name: "name".to_string(),
                    r#type: "String".to_string(),
                    comment: "This is a list of numbers".to_string(),
                    is_optional: false,
                    is_list: false,
                })],
                is_optional: false,
                is_list: true,
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
