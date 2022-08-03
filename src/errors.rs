use std::{
    error::Error,
    fmt::{Debug, Display},
};

#[derive(Debug)]
pub struct RuntimeError(&'static str);
impl RuntimeError {
    pub fn new(message: &'static str) -> Box<dyn Error> {
        let err: Box<dyn Error> = Box::new(Self(message));
        err
    }
    pub fn err<T>(message: &'static str) -> Result<T, Box<dyn Error>> {
        Err(Self::new(message))
    }
}
impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Error for RuntimeError {}
