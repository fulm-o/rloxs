use std::fmt;

#[derive(Debug)]
pub enum EvalError {
    //todo
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}