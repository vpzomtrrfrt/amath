use types::{Context, Expression, Value};

impl Expression {
    pub fn eval(&self, context: &Context) -> Value {
        match *self {
            Expression::Const(ref x) => *x,
            Expression::Add(ref a, ref b) => a.eval(context).add(&b.eval(context)),
            Expression::Subtract(ref a, ref b) => a.eval(context).subtract(&b.eval(context)),
            Expression::Multiply(ref a, ref b) => a.eval(context).multiply(&b.eval(context)),
            Expression::Divide(ref a, ref b) => a.eval(context).divide(&b.eval(context))
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
