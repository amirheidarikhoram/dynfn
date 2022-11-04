#[cfg(test)]
mod tests {
    use dynfn_core::callable::Callable;
    use dynfn_proc_macros::dynfn;
    use serde::{Deserialize, Serialize};
    use serde_json;
    use static_assertions::assert_impl_all;

    #[test]
    fn it_doesnt_panic() {
        #[allow(unused)]
        #[dynfn]
        fn simple() -> String {
            "Hello, world!".to_string()
        }

        assert_eq!(simple::call("".to_string()), Ok("Hello, world!".to_string()));
    }
}
