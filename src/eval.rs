use types::{Context, Expression, Value};
use error::EvalError as Error;

use std::borrow::Cow;

impl Expression {
    pub fn eval<'a>(&'a self, context: &'a Context) -> Result<Cow<'a, Value>, Error> {
        use std::borrow::Borrow;
        match *self {
            Expression::Const(ref x) => Ok(Cow::Borrowed(x)),
            Expression::Add(ref a, ref b) => Ok(Cow::Owned(a.eval(context)?.add(b.eval(context)?.borrow())?)),
            Expression::Subtract(ref a, ref b) => Ok(Cow::Owned(a.eval(context)?.subtract(b.eval(context)?.borrow())?)),
            Expression::Multiply(ref a, ref b) => Ok(Cow::Owned(a.eval(context)?.multiply(b.eval(context)?.borrow(), context)?)),
            Expression::Divide(ref a, ref b) => Ok(Cow::Owned(a.eval(context)?.divide(b.eval(context)?.borrow())?)),
            Expression::Variable(ref n) => context.get(n).ok_or_else(|| Error::UndefinedVariable(n.to_owned())),
            Expression::IsEq(ref a, ref b) => Ok(Cow::Owned(Value::Boolean(a.eval(context)? == b.eval(context)?)))
        }
    }
}

impl Value {
    pub fn multiply(&self, rh: &Value, context: &Context) -> Result<Value, Error> {
        match *self {
            Value::Int(a) => match *rh {
                Value::Int(b) => Ok(Value::Int(a * b)),
                Value::Float(_) => rh.multiply(self, context),
                Value::Function(_, _) => Err(Error::InvalidOperation("Int + Function")),
                Value::Boolean(_) => Err(Error::InvalidOperation("Int * Boolean"))
            },
            Value::Float(a) => match *rh {
                Value::Float(b) => Ok(Value::Float(a * b)),
                Value::Int(b) => Ok(Value::Float(a * f64::from(b))),
                Value::Function(_, _) => Err(Error::InvalidOperation("Float * Function")),
                Value::Boolean(_) => Err(Error::InvalidOperation("Float * Boolean"))
            },
            Value::Function(ref name, ref id) => context.call_function(&id, vec![rh])
                .map_err(|msg| Error::FunctionError(name.to_owned(), msg)),
            Value::Boolean(_) => Err(Error::InvalidOperation("Booleans cannot be multiplied."))
        }
    }
    fn add(&self, rh: &Value) -> Result<Value, Error> {
        match *self {
            Value::Int(a) => match *rh {
                Value::Int(b) => Ok(Value::Int(a + b)),
                Value::Float(_) => rh.add(self),
                Value::Function(_, _) => Err(Error::InvalidOperation("Int + Function")),
                Value::Boolean(_) => Err(Error::InvalidOperation("Int + Boolean"))
            },
            Value::Float(a) => match *rh {
                Value::Float(b) => Ok(Value::Float(a + b)),
                Value::Int(b) => Ok(Value::Float(a + f64::from(b))),
                Value::Function(_, _) => Err(Error::InvalidOperation("Float + Function")),
                Value::Boolean(_) => Err(Error::InvalidOperation("Float + Boolean"))
            },
            Value::Function(_, _) => Err(Error::InvalidOperation("Functions cannot be added.")),
            Value::Boolean(_) => Err(Error::InvalidOperation("Booleans cannot be added."))
        }
    }
    fn subtract(&self, rh: &Value) -> Result<Value, Error> {
        match *self {
            Value::Int(a) => match *rh {
                Value::Int(b) => Ok(Value::Int(a - b)),
                Value::Float(b) => Ok(Value::Float(f64::from(a) - b)),
                Value::Function(_, _) => Err(Error::InvalidOperation("Int - Function")),
                Value::Boolean(_) => Err(Error::InvalidOperation("Int - Boolean"))
            },
            Value::Float(a) => match *rh {
                Value::Float(b) => Ok(Value::Float(a - b)),
                Value::Int(b) => Ok(Value::Float(a - f64::from(b))),
                Value::Function(_, _) => Err(Error::InvalidOperation("Float - Function")),
                Value::Boolean(_) => Err(Error::InvalidOperation("Float - Boolean"))
            },
            Value::Function(_, _) => Err(Error::InvalidOperation("Functions cannot be subtracted.")),
            Value::Boolean(_) => Err(Error::InvalidOperation("Booleans cannot be subtracted."))
        }
    }
    fn divide(&self, rh: &Value) -> Result<Value, Error> {
        match *self {
            Value::Int(a) => match *rh {
                Value::Int(b) => Ok(Value::Int(a / b)),
                Value::Float(b) => Ok(Value::Float(f64::from(a) / b)),
                Value::Function(_, _) => Err(Error::InvalidOperation("Int / Function")),
                Value::Boolean(_) => Err(Error::InvalidOperation("Int / Boolean"))
            },
            Value::Float(a) => match *rh {
                Value::Float(b) => Ok(Value::Float(a / b)),
                Value::Int(b) => Ok(Value::Float(a / f64::from(b))),
                Value::Function(_, _) => Err(Error::InvalidOperation("Float / Function")),
                Value::Boolean(_) => Err(Error::InvalidOperation("Float / Boolean"))
            },
            Value::Function(_, _) => Err(Error::InvalidOperation("Functions cannot be divided.")),
            Value::Boolean(_) => Err(Error::InvalidOperation("Booleans cannot be divided."))
        }
    }
}
