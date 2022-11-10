use std::error::Error;
use serde::{Serialize};
#[derive(Serialize, Debug)]
pub struct Success<T> {
    code: u8,
    data: T
}
#[derive(Serialize, Debug)]
pub struct Fail {
    code: u8,
    data: String
}
#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum Respone<T> {
    Success(Success<T>),
    Fail(Fail)
}
impl<T> Respone<T> {
    pub fn ok(data: T) -> Self {
        Self::Success(Success {
            code: 0,
            data: data
        })
    }
    pub fn err(err: Box<dyn Error>) -> Self {
        Self::Fail(Fail {
            code: 1,
            data: err.to_string()
        })
    }
}

impl<T> From<Result<T, Box<dyn Error>>> for Respone<T> {
    fn from(ret: Result<T, Box<dyn Error>>) -> Self {
        match ret {
            Ok(t) => {
                return Self::ok(t)
            }
            Err(e) => {
                return Self::err(e)
            }
        }
    }
}