#[derive(Debug)]
pub enum Expression {
    Const(Value),
    Multiply(Box<Expression>, Box<Expression>)
}

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Int(i32)
}

#[derive(Default)]
pub struct Context {}
