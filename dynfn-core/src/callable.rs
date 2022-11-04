use std::io::Result;
use serde::{Serialize, Deserialize};

pub trait Callable<'a, T, R> where T: Serialize + Deserialize<'a>, R: Serialize + Deserialize<'a> {
    fn call(args: T) -> Result<R>;
}