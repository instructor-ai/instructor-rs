# Response Model

Defining LLM Output Schemas in Instructor is done via our `InstructMacro` macro which generates the underlying JSON Schema from your Rust Struct. We support a variety of different types out of the box like Enums, Vecs and Option types, read more about how to use them [here](/types).

## What happens behind the scenes?

At Runtime, Rust compiles your structs into blocks of memory with specific offsets for each field.

Accessing fields involves using these offsets, similar to accessing array elements with indices. This approach ensures that structs are efficient and have no additional runtime overhead compared to manual memory management techniques.

```rust
struct Example {
    a: u32,
    b: u64,
    c: u8,
}
```

This means that we lose a significant amount of information about the types and fields that you use in your code. When we use the `InstructMacro`, we rewrite your struct under the hood to expose a `get_info()` method which contains information on your struct.

```rust
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
```

We add this code under the hood ( You can view all the expanded code using `cargo-expand` with the command `cargo expand <file name>`).

```rust
impl instruct_macros_types::InstructMacro for TestStruct {
    fn get_info() -> instruct_macros_types::InstructMacroResult {
        let mut parameters = Vec::new();
        parameters
            .push(
                Parameter::Field(ParameterInfo {
                    name: "field1".to_string(),
                    r#type: "String".to_string(),
                    comment: "This is a sample example that spans across three lines"
                        .to_string(),
                    is_optional: false,
                    is_list: false,
                }),
            );
        parameters
            .push(
                Parameter::Field(ParameterInfo {
                    name: "field2".to_string(),
                    r#type: "str".to_string(),
                    comment: "This is a test field".to_string(),
                    is_optional: false,
                    is_list: false,
                }),
            );
        instruct_macros_types::InstructMacroResult::Struct(StructInfo {
            name: "TestStruct".to_string(),
            description: "This is a struct".to_string(),
            parameters,
            is_optional: false,
            is_list: false,
        })
    }
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
```
