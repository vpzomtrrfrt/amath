use std;

pub enum EvalError {
    UndefinedVariable(String),
    InvalidOperation(&'static str),
    FunctionError(String, String)
}

impl std::fmt::Debug for EvalError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self, fmt)
    }
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", match *self {
            EvalError::UndefinedVariable(ref n) => format!("Undefined variable: {}", n),
            EvalError::InvalidOperation(ref t) => format!("Invalid operation: {}", t),
            EvalError::FunctionError(ref n, ref m) => format!("Error in {}: {}", n, m)
        })
    }
}

impl std::error::Error for EvalError {
    fn description(&self) -> &str {
        match *self {
            EvalError::UndefinedVariable(_) => "Undefined variable",
            EvalError::InvalidOperation(_) => "Invalid operation",
            EvalError::FunctionError(_, _) => "Error in function"
        }
    }
}
