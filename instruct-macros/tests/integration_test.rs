extern crate instruct_macros_types;

use instruct_macros::{validate, InstructMacro};
use instruct_macros_types::{InstructMacro, ParameterInfo, StructInfo};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_conversion() {
        #[derive(InstructMacro, Debug)]
        #[allow(dead_code)]
        struct TestStruct {
            ///This is a test field
            field1: String,
            ///This is a test field
            field2: str,
        }
        let info = TestStruct::get_info();
        let desired_struct = StructInfo {
            name: "TestStruct".to_string(),
            description: "".to_string(),
            parameters: vec![
                ParameterInfo {
                    name: "field1".to_string(),
                    r#type: "String".to_string(),
                    comment: "This is a test field".to_string(),
                },
                ParameterInfo {
                    name: "field2".to_string(),
                    r#type: "str".to_string(),
                    comment: "This is a test field".to_string(),
                },
            ],
        };
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
}
