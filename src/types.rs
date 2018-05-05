use std;

#[derive(Debug)]
pub enum Expression {
    Const(Value),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Variable(String)
}

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Int(i32),
    Float(f64)
}

#[derive(Default)]
pub struct Context {
    map: std::collections::HashMap<String, Value>
}

impl Context {
    pub fn set(&mut self, name: String, value: Value) {
        self.map.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<std::borrow::Cow<Value>> {
        self.map.get(name).and_then(|x| Some(std::borrow::Cow::Borrowed(x)))
    }
}
