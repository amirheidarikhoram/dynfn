#[cfg(test)]
mod tests {
    use dynfn_proc_macros::dynfn;
    use serde::{Deserialize, Serialize};
    use serde_json;
    use static_assertions::assert_impl_all;

    #[test]
    fn it_works_wihtout_args() {
        #[allow(unused)]
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
        #[allow(unused)]
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

        #[allow(unused)]
        #[dynfn]
        fn simple_with_serde(person: Person) -> String {
            format!("Hello, {}!", person.name)
        }

        assert_eq!(
            simple_with_serde(Some("{\"person\": {\"name\": \"John\", \"age\": 42}}".to_string())),
            Ok("\"Hello, John!\"".to_string())
        );
    }
}
