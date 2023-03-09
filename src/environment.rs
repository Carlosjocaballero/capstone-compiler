use std::collections::HashMap;
use std::fmt::format;
use crate::token::*;
use crate::LoxError::*;

#[derive(PartialEq, Clone)]
pub struct Environment {
    pub enclosing: Option<Box<Environment>>,
    pub values: HashMap<String, Literal>,
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
            enclosing: Some(Box::new(enclosing)),
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

    pub fn get(&mut self, name: &Token) -> Literal{
        if self.values.contains_key(&name.lexeme){
            return match self.values.get(&name.lexeme){
                Some(x) => x.clone(),
                None => Literal::None
            }
        }
        
        if self.enclosing != None{
            return Option::expect(self.enclosing.clone(), "").get(&name);
        }
        else{
            panic!()
        }
    }

    pub fn assign(&mut self, name: Token, value: &Literal){
        //println!("{:?}", self.values);
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

    pub fn print_map(&mut self){
        println!("{:?}", self.values);
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