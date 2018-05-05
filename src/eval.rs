use types::{Context, Expression, Value};
use error::EvalError as Error;

use std::borrow::Cow;

impl Expression {
    pub fn eval<'a>(&'a self, context: &'a Context) -> Result<Cow<'a, Value>, Error> {
        use std::borrow::Borrow;
        match *self {
            Expression::Const(ref x) => Ok(Cow::Borrowed(x)),
            Expression::Add(ref a, ref b) => Ok(Cow::Owned(a.eval(context)?.add(b.eval(context)?.borrow()))),
            Expression::Subtract(ref a, ref b) => Ok(Cow::Owned(a.eval(context)?.subtract(b.eval(context)?.borrow()))),
            Expression::Multiply(ref a, ref b) => Ok(Cow::Owned(a.eval(context)?.multiply(b.eval(context)?.borrow()))),
            Expression::Divide(ref a, ref b) => Ok(Cow::Owned(a.eval(context)?.divide(b.eval(context)?.borrow()))),
            Expression::Variable(ref n) => context.get(n).ok_or_else(|| Error::UndefinedVariable(n.to_owned()))
        }
    }
}

impl Value {
    fn multiply(&self, rh: &Value) -> Value {
        match *self {
            Value::Int(a) => match *rh {
                Value::Int(b) => Value::Int(a * b),
                Value::Float(_) => rh.multiply(self)
            },
            Value::Float(a) => match *rh {
                Value::Float(b) => Value::Float(a * b),
                Value::Int(b) => Value::Float(a * f64::from(b))
            }
        }
    }
    fn add(&self, rh: &Value) -> Value {
        match *self {
            Value::Int(a) => match *rh {
                Value::Int(b) => Value::Int(a + b),
                Value::Float(_) => rh.add(self)
            },
            Value::Float(a) => match *rh {
                Value::Float(b) => Value::Float(a + b),
                Value::Int(b) => Value::Float(a + f64::from(b))
            }
        }
    }
    fn subtract(&self, rh: &Value) -> Value {
        match *self {
            Value::Int(a) => match *rh {
                Value::Int(b) => Value::Int(a - b),
                Value::Float(b) => Value::Float(f64::from(a) - b)
            },
            Value::Float(a) => match *rh {
                Value::Float(b) => Value::Float(a - b),
                Value::Int(b) => Value::Float(a - f64::from(b))
            }
        }
    }
    fn divide(&self, rh: &Value) -> Value {
        match *self {
            Value::Int(a) => match *rh {
                Value::Int(b) => Value::Int(a / b),
                Value::Float(b) => Value::Float(f64::from(a) / b)
            },
            Value::Float(a) => match *rh {
                Value::Float(b) => Value::Float(a / b),
                Value::Int(b) => Value::Float(a / f64::from(b))
            }
        }
    }
}
