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

    pub fn new_enclosed(enclosing : &Environment) -> Self{
        Environment { 
            enclosing: Some(Box::new(enclosing.clone())),
            values: enclosing.values.clone(),
            error: InterpreterError { is_error: false } 
            }
    }

    pub fn define(&mut self, name: String, value: Literal){
        self.values.insert(name, value);
    }

    pub fn ancestor(&self, distance: i32) -> Environment{
        let mut environment = self;
        let mut temp: Box<Environment>;
        for i in 0..distance{
            match &environment.enclosing.clone(){
                Some(environmentEnclosing) => {
                    temp = environmentEnclosing.clone();
                    environment = &*temp;
                },
                None => ()
            }
        }
        return environment.clone();
    }

    pub fn getAt(&self, distance: i32, name: String) -> Result<Literal, ScannerError>{
        match self.ancestor(distance).values.get(&name){
            Some(x) => return Ok(x.clone()),
            None => return Ok(Literal::None)
        }
    }

    pub fn assignAt(&self, distance: i32, name: Token, value: Literal){
        self.ancestor(distance).values.insert(name.lexeme, value);
    }

    pub fn get(&mut self, name: &Token) -> Literal{
        if self.values.contains_key(&name.lexeme){
            return match self.values.get(&name.lexeme){
                Some(x) => x.clone(),
                None => Literal::None
            }
        }
        
        if self.enclosing != None{
            // return Option::expect(self.enclosing.clone(), "").get(&name);
            match self.enclosing.clone(){
                Some(mut x) => x.get(name),
                None => Literal::None
            }
        }
        else{
            println!{"environemnt.rs:get():53\n{:?}", self.values};
            panic!()
        }
    }

    pub fn assign(&mut self, name: Token, value: &Literal){
        //println!("Environemnt:assign():56\nmap: {:?}\nvariable name: {:?}\nnew value: {:?}", self.values, name.lexeme,value);
        if self.values.contains_key(&name.lexeme){
            self.values.insert(name.lexeme.clone(), value.clone());
            //self.print_map();
            return;
        }

        let name_copy = name.clone();
        if self.enclosing != None {
            // let mut x = Option::unwrap(self.enclosing);
            // x.assign(name, value);
            // //self.print_map();
            // return;
            Option::expect(self.enclosing.clone(), "Couldn't assign").assign(name, value);
            println!("environment:assign():70\n");
            self.print_map();
            return;
        }   

        self.error.run_time_error(&name_copy, format!("Undefined variable '{}'.", name_copy.lexeme));
    }

    pub fn print_map(&mut self){
        println!("{:?}", self.values);
    }
}