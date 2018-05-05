#[derive(Debug)]
pub enum Expression {
    Const(Value),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>)
}

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Int(i32)
}

#[derive(Default)]
pub struct Context {}
