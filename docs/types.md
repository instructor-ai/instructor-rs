# Support for Simple Types

We support most basic types out of the box with our `InstructMacro` macro now. Here's a quick rundown of how to use our macro.

## Importing the Macro

You'll need to use the following imports to use the macro

```rust
use instruct_macros::{InstructMacro};
use instruct_macros_types::{
    InstructMacro, InstructMacroResult, Parameter, ParameterInfo, StructInfo,
};

 #[derive(InstructMacro, Debug)]
struct User {
    pub name: String,
    pub age: String
}

/*
{
  "type": "object",
  "properties": {
    "age": {
      "type": "string",
      "description": ""
    },
    "name": {
      "type": "string",
      "description": ""
    }
  },
  "required": [
    "name",
    "age"
  ]
}
*/
```

### Adding a Description

We provide a `#[description( Description goes here )]` annoration that you can add to your struct. This will be included in the function call which we will send over to OpenAI/other inference providers.

This is the same for individual fields or the entire struct with multi-line comments being relatively easy to implement.

```rust
#[derive(InstructMacro, Debug)]
#[description("This is a user object")]
struct User {
    #[description("This is the name of the user")]
    pub name: String,
    #[description(
        "This is\
    a multi-line description\
    which can be used"
    )]
    pub age: String,
}

/*
{
  "type": "object",
  "properties": {
    "name": {
      "type": "string",
      "description": "This is the name of the user"
    },
    "age": {
      "type": "string",
      "description": "This isa multi-line descriptionwhich can be used"
    }
  },
  "required": [
    "name",
    "age"
  ]
}
*/
```

## Advanced Types

### Enums

Enums are supported in the same way. Just declare it as if you would a normal `Serde` object and it'll work out of the box seamlessly.

```rust
#[derive(InstructMacro, Debug)]
#[description("This is an enum representing the status of a person")]
pub enum Status {
    Active,
    Inactive,
    Pending,
}

#[derive(InstructMacro, Debug)]
pub struct User {
    name: String,
    status: Status,
}

/*
{
  "type": "object",
  "properties": {
    "name": {
      "type": "string",
      "description": ""
    },
    "status": {
      "type": "string",
      "description": "This is an enum representing the status of a person",
      "enum_values": [
        "Active",
        "Inactive",
        "Pending"
      ]
    }
  },
  "required": [
    "name",
    "status"
  ]
}
*/
```

If you'll like to provide a custom description for your enum field in your struct, just use the `description` annoration and we'll override the default description of the enum when we generate the function parameter.

```rust
#[derive(InstructMacro, Debug)]
#[description("This is an enum representing the status of a person")]
pub enum Status {
    Active,
    Inactive,
    Pending,
}

#[derive(InstructMacro, Debug)]
pub struct User {
    name: String,
    #[description("This is the person's status")]
    status: Status,
}

/*
{
  "type": "object",
  "properties": {
    "status": {
      "type": "string",
      "description": "This is the person's status",
      "enum_values": [
        "Active",
        "Inactive",
        "Pending"
      ]
    },
    "name": {
      "type": "string",
      "description": ""
    }
  },
  "required": [
    "name",
    "status"
  ]
}
*/
```

### Vectors

Sometimes you might want to extract a list of objects (Eg. Users). To do so, you can just use a simple `Vec` object.

```rust
#[derive(InstructMacro, Debug)]
#[description("This is a struct with Option types")]
struct Numbers {
    #[description("This is a list of numbers")]
    pub numbers: Vec<i32>,
}

/*
{
  "type": "object",
  "properties": {
    "users": {
      "type": "array",
      "description": "A list of users",
      "items": {
        "type": "object",
        "properties": {
          "name": {
            "type": "string",
            "description": ""
          }
        }
      }
    }
  },
  "required": [
    "users"
  ]
}
*/
```

### Options

We also support Option types. This is most popular when using a `Maybe` pattern where we have some form of data that we might want to extract.

```rust
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
    error_message: Option<String>
}

/*
{
  "type": "object",
  "properties": {
    "user": {
      "type": "object",
      "description": "This is an optional user field. If the user is not present, the field will be null",
      "properties": {
        "age": {
          "type": "number",
          "description": ""
        },
        "name": {
          "type": "string",
          "description": ""
        }
      }
    },
    "error_message": {
      "type": "string",
      "description": ""
    }
  },
  "required": []
}
*/
```
