pub enum IsError{
    True,
    False
}

pub struct Error{
    is_error: IsError
}

impl Error{
    pub fn _error(&self, line :i32, message: String){
        self.report(line, "".to_string(), message)
    }

    pub fn report(&self, line: i32, _where: String, message: String){
        println!("[line {}] Error {}: {}", line, _where, message);
        self.error_true()
    }

    pub fn error_false(&self){
        self::IsError::False;
    }

    pub fn error_true(&self){
        self::IsError::True;
    }

}

