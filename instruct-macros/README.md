# Instruct Macros

Instruct Macros are a set of procedural macros that expose a `get_info()` method which is meant to help you get reflection in your objects by default, exposing field names+ types along with the object name.

You can use it by doing

```rust
use instruct_macros::InstructMacro; // Ensure this is a derive macro
use instruct_macros_types::{ParameterInfo, StructInfo}; // Import the trait

#[derive(InstructMacro, Deserialize, Serialize, Debug)]
/// This is a model which represents a single individual user
struct UserInfo {
    /// This is the name of the user
    #[serde(deserialize_with = "uppercase_name")]
    name: String,
    /// This is the age of the user
    age: u8,
    /// This is the city of the user
    city: String,
}
```

This in turn will expose a get_info() method on your struct that returns a body that looks omsething like this
