extern crate instruct_macros_types;

use instruct_macros::InstructMacro; // Add this line
use instruct_macros_types::{InstructMacro, ParameterInfo, StructInfo};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_conversion() {
        #[derive(InstructMacro, Debug)]
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
}
