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

#[derive(Debug, Clone)]
pub enum Value {
    Int(i32),
    Float(f64),
    Function(String, u64),
    Boolean(bool)
}

#[derive(Default)]
pub struct Context {
    map: std::collections::HashMap<String, Value>,
    func_map: std::collections::HashMap<u64, Box<Fn(Vec<&Value>, &Context) -> Result<Value, String>>>
}

impl Context {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set(&mut self, name: String, value: Value) {
        self.map.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<std::borrow::Cow<Value>> {
        self.map.get(name).and_then(|x| Some(std::borrow::Cow::Borrowed(x)))
    }

    pub fn add_function<F: 'static + Fn(Vec<&Value>, &Context) -> Result<Value, String>>(
        &mut self,
        name: String,
        f: F
    ) {
        use std::hash::Hasher;

        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(&name, &mut hasher);
        let id = hasher.finish();
        self.func_map.insert(id, Box::new(f));
        self.map.insert(name.clone(), Value::Function(name, id));
    }

    pub fn call_function(&self, id: &u64, params: Vec<&Value>) -> Result<Value, String> {
        let f = self.func_map.get(id).ok_or_else(|| "No such function".to_owned())?;
        f(params, self)
    }
}
