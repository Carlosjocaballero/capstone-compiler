use std::collections::HashMap;
use std::fmt::format;
use crate::token::*;
use crate::LoxError::*;

#[derive(PartialEq, Clone)]
pub struct Environment {
    pub enclosing: Option<Box<Environment>>,
    values: HashMap<String, Literal>,
    error: InterpreterError,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
            error: InterpreterError { is_error: false },
            enclosing: None,
        }
    }

    pub fn new_enclosed(enclosing : Environment) -> Self{
        Environment { 
            enclosing: enclosing.enclosing,
            values: HashMap::new(),
            error: InterpreterError { is_error: false } 
            }
    }

    // fn with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
    //     Environment {
    //         values: HashMap::new(),
    //         enclosing: Some(enclosing),
    //     }
    // }

    pub fn define(&mut self, name: String, value: Literal){
        self.values.insert(name, value);
    }

    pub fn ancestor(&self, distance: i32) -> Environment{
        let environment = self;
        for i in 0..distance{
            match environment.enclosing{
                Some(environmentEnclosing) => environment = &environmentEnclosing,
                None => ()
            }
        }
        return *environment;
    }

    //book wants name to be a string, but the get function only takes in a reference to a Token
    //Don't think it's possible to change a string to a Token without making a whole new Token which seems counterproductive
    pub fn getAt(&self, distance: i32, name: Token) -> Result<Literal, ScannerError>{
        return Ok(self.ancestor(distance).get(&name));
    }

    pub fn get(&mut self, name: &Token) -> Literal{
        match self.values.get(&name.lexeme){
            Some(value) => return value.clone(),
            None => {
                if self.enclosing != None{
                    let mut x: Box<Environment> = Option::unwrap(self.enclosing.clone());
                    return x.get(name)
                }
                self.error.run_time_error(name, format!("Undefined variable '{}'.", name.lexeme));
                Literal::None
            }
        }
    }

    pub fn assign(&mut self, name: Token, value: &Literal){
        if self.values.contains_key(&name.lexeme){
            self.values.insert(name.lexeme, value.clone());
            return;
        }

        if self.enclosing != None {
            let mut x = Option::unwrap(self.enclosing.clone());
            x.assign(name, value);
            return;
        }   

        self.error.run_time_error(&name, format!("Undefined variable '{}'.", name.lexeme));
    }
}

// fn get(&self, name: &Token) -> Result<Object, RuntimeError> {
//     match self.values.get(&name.lexeme) {
//         Some(value) => Ok(value.clone()),
//         None => Err(RuntimeError::new(
//             name.clone(),
//             format!("Undefined varable '{}'", name.lexeme)
//         )),
//     }
//     if let Some(enclosing) = &self.enclosing {
//         return enclosing.get(name);
//     }
// }

// fn assign(&mut self, name: &Token, value: Object) -> Result<(), RuntimeError> {
//     if self.values.contains_key(&name.lexeme) {
//         self.values.insert(name.lexeme.clone(), value);
//         Ok(())
//     } else {
//         Err(RuntimeError::new(
//             name.clone(),
//             format!("Undefined variable '{}'.", name.lexeme)
//         ))
//     }

//     if let Some(enclosing) = &self.enclosing {
//         enclosing.assign(name, value);
//         return;
//     }
// }

// fn define_env(&mut self, name: String, value: Literal) {
//     self.values.insert(name, value);
// }