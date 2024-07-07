extern crate instruct_macros_types;

use instruct_macros::{validate, InstructMacro};
use instruct_macros_types::{InstructMacro, Parameter, ParameterInfo, StructInfo};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_conversion() {
        #[derive(InstructMacro, Debug)]
        #[allow(dead_code)]
        // This is a struct
        struct TestStruct {
            #[description(
                "This is a sample example \
                that spans across \
                three lines"
            )]
            pub field1: String,
            #[description("This is a test field")]
            pub field2: str,
        }
        let info = TestStruct::get_info();
        let desired_struct = StructInfo {
            name: "TestStruct".to_string(),
            description: "".to_string(),
            parameters: vec![
                Parameter::Field(ParameterInfo {
                    name: "field1".to_string(),
                    r#type: "String".to_string(),
                    comment: "This is a sample example that spans across three lines".to_string(),
                }),
                Parameter::Field(ParameterInfo {
                    name: "field2".to_string(),
                    r#type: "str".to_string(),
                    comment: "This is a test field".to_string(),
                }),
            ],
        };
        println!("Info: {:?}", info);
        println!("Desired Struct: {:?}", desired_struct);
        assert!(info == desired_struct);
    }

    #[test]
    fn test_validation_macro() {
        #[derive(InstructMacro, Debug)]
        pub struct UserInfo {
            #[validate(custom = "validate_uppercase")]
            pub name: String,
            pub age: u8,
        }

        #[validate]
        fn validate_uppercase(name: &String) -> Result<String, String> {
            if name.chars().any(|c| c.is_lowercase()) {
                return Err(format!(
                    "Name '{}' should be entirely in uppercase. Examples: 'TIMOTHY', 'JANE SMITH'",
                    name
                ));
            }
            Ok(name.to_uppercase())
        }

        let user_info = UserInfo {
            name: "JoHn DoE".to_string(),
            age: 100,
        };

        assert_eq!(
            user_info.validate().unwrap_err(),
            "Validation failed for field 'name': Name 'JoHn DoE' should be entirely in uppercase. Examples: 'TIMOTHY', 'JANE SMITH'"
        );

        let user_info = UserInfo {
            name: "JOHN DOE".to_string(),
            age: 30,
        };

        assert!(user_info.validate().is_ok());
    }

    #[test]
    fn test_nested_struct_macro() {
        #[derive(InstructMacro, Debug)]
        pub struct Address {
            pub street: String,
            pub city: String,
        }

        #[derive(InstructMacro, Debug)]
        pub struct User {
            pub name: String,
            pub age: u8,
            pub address: Address,
        }

        let info = User::get_info();
        let desired_struct = StructInfo {
            name: "User".to_string(),
            description: "".to_string(),
            parameters: vec![
                Parameter::Field(ParameterInfo {
                    name: "name".to_string(),
                    r#type: "String".to_string(),
                    comment: "".to_string(),
                }),
                Parameter::Field(ParameterInfo {
                    name: "age".to_string(),
                    r#type: "u8".to_string(),
                    comment: "".to_string(),
                }),
                Parameter::Struct(StructInfo {
                    name: "address".to_string(),
                    description: "".to_string(),
                    parameters: vec![
                        Parameter::Field(ParameterInfo {
                            name: "street".to_string(),
                            r#type: "String".to_string(),
                            comment: "".to_string(),
                        }),
                        Parameter::Field(ParameterInfo {
                            name: "city".to_string(),
                            r#type: "String".to_string(),
                            comment: "".to_string(),
                        }),
                    ],
                }),
            ],
        };

        println!("{:?}", desired_struct);
        println!("{:?}", info);
        assert!(info == desired_struct);
    }
}
