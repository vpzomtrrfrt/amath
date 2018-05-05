use types::{Context, Expression, Value};

impl Expression {
    pub fn eval(&self, context: &Context) -> Value {
        match *self {
            Expression::Const(ref x) => *x,
            Expression::Add(ref a, ref b) => a.eval(context).add(b.eval(context)),
            Expression::Subtract(ref a, ref b) => a.eval(context).subtract(b.eval(context)),
            Expression::Multiply(ref a, ref b) => a.eval(context).multiply(b.eval(context)),
            Expression::Divide(ref a, ref b) => a.eval(context).divide(b.eval(context))
        }
    }
}

impl Value {
    fn multiply(&self, rh: Value) -> Value {
        match *self {
            Value::Int(a) => match rh {
                Value::Int(b) => Value::Int(a * b)
            }
        }
    }
    fn add(&self, rh: Value) -> Value {
        match *self {
            Value::Int(a) => match rh {
                Value::Int(b) => Value::Int(a + b)
            }
        }
    }
    fn subtract(&self, rh: Value) -> Value {
        match *self {
            Value::Int(a) => match rh {
                Value::Int(b) => Value::Int(a - b)
            }
        }
    }
    fn divide(&self, rh: Value) -> Value {
        match *self {
            Value::Int(a) => match rh {
                Value::Int(b) => Value::Int(a / b)
            }
        }
    }
}
