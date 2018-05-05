use std;

pub enum EvalError {
    UndefinedVariable(String)
}

impl std::fmt::Debug for EvalError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self, fmt)
    }
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", match *self {
            EvalError::UndefinedVariable(ref n) => format!("Undefined variable: {}", n)
        })
    }
}

impl std::error::Error for EvalError {
    fn description(&self) -> &str {
        match *self {
            EvalError::UndefinedVariable(_) => "Undefined variable"
        }
    }
}
