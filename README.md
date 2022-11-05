# dynfn

dyfn is a tool to change rust function signature to `fn (Option<String>) -> Result<String, ()>` to be able to call it with corresponding JSON data regardless of its actual signature in rust context.

## dynfn in action
```rust
use dynfn::prelude::*;

#[dynfn]
fn sum (a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let json = r#"{"a": 1, "b": 2}"#;
    let result = sum(json);
    assert_eq!(result, Ok("3".to_string()));
}
```

## Why?
In rust it is currently not possible to store function pointers of functions in collections such as Vec which all function signatures don't match. Or generally you may face issues working with function pointers with signatures that are not the same. A workaround for this issue is to somehow convert that function to a new function which accepts a sort of data that can be used to call the former (inner, wrapped) function.

## How it works
dynfn internally uses [serde](https://crates.io/crates/serde) and [serde_json](https://crates.io/crates/serde_json) in order to serialize and deserialize JSON data, calls the inner function with the right data and returns the response. All function arguments must be named and their corresponding types must implement `serde::Serialize` and `serde::Deserialize` traits. Note that function return type must implement those traits as well.

## Limitations and Possible Issues
The solution is not perfect and has some limitations. For example, it is not possible to call a function with a reference argument. However, it is possible to call a function with a struct argument as long as the struct implements `serde::Serialize` and `serde::Deserialize` traits.
The other issues that you may (or may not) face are performance issues. Performance issues are caused by the fact that dynfn uses serde to serialize and deserialize JSON data. However, this is not a big issue in most cases.

## Contribution
Any contribution is welcome. Please feel free to open an issue or a pull request.