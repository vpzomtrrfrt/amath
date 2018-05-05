use types::{Context, Expression, Value};

impl Expression {
    pub fn eval(&self, context: &Context) -> Value {
        match *self {
            Expression::Const(ref x) => *x,
            Expression::Multiply(ref a, ref b) => a.eval(context).multiply(b.eval(context))
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
}
