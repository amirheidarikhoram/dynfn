pub mod prelude;

#[cfg(test)]
mod tests {
    use super::prelude::*;

    #[test]
    fn it_works_wihtout_args() {
        #[dynfn]
        fn simple() -> String {
            "Hello, world!".to_string()
        }

        assert_eq!(
            simple(None),
            Ok("\"Hello, world!\"".to_string())
        );
    }

    #[test]
    fn it_works_with_args() {
        #[dynfn]
        fn simple_with_args(a: u32, b: u32) -> u32 {
            a + b
        }

        assert_eq!(
            simple_with_args(Some("{\"a\": 1, \"b\": 2}".to_string())),
            Ok("3".to_string())
        );
    }

    #[test]
    fn it_works_with_serde() {
        #[derive(Serialize, Deserialize)]
        struct Person {
            name: String,
            age: u32,
        }

        #[dynfn]
        fn simple_with_serde(person: Person) -> String {
            format!("Hello, {}!", person.name)
        }

        assert_eq!(
            simple_with_serde(Some("{\"person\": {\"name\": \"John\", \"age\": 42}}".to_string())),
            Ok("\"Hello, John!\"".to_string())
        );
    }

    #[test]
    fn it_works_with_serde_and_args() {
        #[derive(Serialize, Deserialize)]
        struct Person {
            name: String,
            age: u32,
        }

        #[dynfn]
        fn simple_with_serde_and_args(person: Person, a: u32, b: u32) -> String {
            format!("Hello, {}! {} + {} = {}", person.name, a, b, a + b)
        }

        assert_eq!(
            simple_with_serde_and_args(Some("{\"person\": {\"name\": \"John\", \"age\": 42}, \"a\": 1, \"b\": 2}".to_string())),
            Ok("\"Hello, John! 1 + 2 = 3\"".to_string())
        );
    }

    #[test]
    fn calls_array_of_function_on_args () {
        #[dynfn]
        fn sum(a: i32, b: i32) -> i32 {
            a + b
        }

        #[dynfn]
        fn mul(a: i32, b: i32) -> i32 {
            a * b
        }

        let mut functions: Vec<DynamicFunction> = Vec::new();
        functions.push(sum);
        functions.push(mul);

        assert_eq!(
            functions[0](Some("{\"a\": 1, \"b\": 2}".to_string())),
            Ok("3".to_string())
        );
    }
}
