use serde::{Serialize, Deserialize};
use std::result::Result;

pub trait Callable<T, R> where T: Serialize + Deserialize<'static>, R: Serialize + Deserialize<'static> {
    fn call(args: T) -> Result<R, ()>;
}