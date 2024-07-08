extern crate instruct_macros_types;

use instruct_macros::{validate, InstructMacro};
use instruct_macros_types::{
    InstructMacro, InstructMacroResult, Parameter, ParameterInfo, StructInfo,
};

#[cfg(test)]
mod tests {

    use instruct_macros_types::EnumInfo;

    use super::*;

    #[test]
    fn test_string_conversion() {
        #[derive(InstructMacro, Debug)]
        #[allow(dead_code)]
        #[description("This is a struct")]
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
            description: "This is a struct".to_string(),
            parameters: vec![
                Parameter::Field(ParameterInfo {
                    name: "field1".to_string(),
                    r#type: "String".to_string(),
                    comment: "This is a sample example that spans across three lines".to_string(),
                    is_optional: false,
                    is_list: false,
                }),
                Parameter::Field(ParameterInfo {
                    name: "field2".to_string(),
                    r#type: "str".to_string(),
                    comment: "This is a test field".to_string(),
                    is_optional: false,
                    is_list: false,
                }),
            ],
            is_optional: false,
            is_list: false,
        };

        let info_struct = match info {
            InstructMacroResult::Struct(s) => s,
            _ => panic!("Expected StructInfo"),
        };

        println!("info_struct: {:?}", info_struct);
        println!("desired_struct: {:?}", desired_struct);
        assert!(info_struct == desired_struct);
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
                    is_optional: false,
                    is_list: false,
                }),
                Parameter::Field(ParameterInfo {
                    name: "age".to_string(),
                    r#type: "u8".to_string(),
                    comment: "".to_string(),
                    is_optional: false,
                    is_list: false,
                }),
                Parameter::Struct(StructInfo {
                    name: "address".to_string(),
                    description: "".to_string(),
                    parameters: vec![
                        Parameter::Field(ParameterInfo {
                            name: "street".to_string(),
                            r#type: "String".to_string(),
                            comment: "".to_string(),
                            is_optional: false,
                            is_list: false,
                        }),
                        Parameter::Field(ParameterInfo {
                            name: "city".to_string(),
                            r#type: "String".to_string(),
                            comment: "".to_string(),
                            is_optional: false,
                            is_list: false,
                        }),
                    ],
                    is_optional: false,
                    is_list: false,
                }),
            ],
            is_optional: false,
            is_list: false,
        };

        let info_struct = match info {
            InstructMacroResult::Struct(s) => s,
            _ => panic!("Expected StructInfo"),
        };
        assert!(info_struct == desired_struct);
    }

    #[test]
    fn test_single_enum() {
        #[derive(InstructMacro, Debug)]
        #[allow(dead_code)]
        #[description = "This is an enum representing the status of a person"]
        pub enum Status {
            Active,
            Inactive,
            Pending,
        }

        let info = Status::get_info();

        let desired_enum = EnumInfo {
            title: "Status".to_string(),
            r#enum: vec![
                "Active".to_string(),
                "Inactive".to_string(),
                "Pending".to_string(),
            ],
            r#type: "Status".to_string(),
            description: "".to_string(),
            is_optional: false,
            is_list: false,
        };

        let info_enum = match info {
            InstructMacroResult::Enum(e) => e,
            _ => panic!("Expected EnumInfo"),
        };

        assert!(info_enum == desired_enum);
    }

    #[test]
    fn test_enum_as_struct_property() {
        #[derive(InstructMacro, Debug)]
        #[allow(dead_code)]
        #[description("This is an enum representing the status of a person")]
        pub enum Status {
            Active,
            Inactive,
            Pending,
        }

        #[derive(InstructMacro, Debug)]
        #[allow(dead_code)]
        pub struct User {
            name: String,
            status: Status,
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
                    is_optional: false,
                    is_list: false,
                }),
                Parameter::Enum(EnumInfo {
                    title: "status".to_string(),
                    r#enum: vec![
                        "Active".to_string(),
                        "Inactive".to_string(),
                        "Pending".to_string(),
                    ],
                    r#type: "Status".to_string(),
                    description: "This is an enum representing the status of a person".to_string(),
                    is_optional: false,
                    is_list: false,
                }),
            ],
            is_optional: false,
            is_list: false,
        };

        let info_struct = match info {
            InstructMacroResult::Struct(s) => s,
            _ => panic!("Expected StructInfo"),
        };

        println!("info_struct: {:?}", info_struct);
        println!("desired_struct: {:?}", desired_struct);

        assert!(info_struct == desired_struct);
    }
}
