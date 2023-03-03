use std::collections::HashMap;

struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: HashMap<String, Object>,
}

impl Environment {
    fn new() -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    fn with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }
}

fn get(&self, name: &Token) -> Result<Object, RuntimeError> {
    match self.values.get(&name.lexeme) {
        Some(value) => Ok(value.clone()),
        None => Err(RuntimeError::new(
            name.clone(),
            format!("Undefined varable '{}'", name.lexeme)
        )),
    }
    if let Some(enclosing) = &self.enclosing {
        return enclosing.get(name);
    }
}

fn assign(&mut self, name: &Token, value: Object) -> Result<(), RuntimeError> {
    if self.values.contains_key(&name.lexeme) {
        self.values.insert(name.lexeme.clone(), value);
        Ok(())
    } else {
        Err(RuntimeError::new(
            name.clone(),
            format!("Undefined variable '{}'.", name.lexeme)
        ))
    }

    if let Some(enclosing) = &self.enclosing {
        enclosing.assign(name, value);
        return;
    }
}

fn define_env(&mut self, name: String, value: Object) {
    self.values.insert(name, value);
}