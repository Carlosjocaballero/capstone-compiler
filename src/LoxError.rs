use crate::Token;

pub struct ScannerError{
    pub is_error: bool
}

impl ScannerError{
    pub fn _error(&mut self, line :u32, message: String){
        self.report(line, "".to_string(), message)
    }

    pub fn report(&mut self, line: u32, _where: String, message: String){
        println!("[line {}] Error {}: {}", line, _where, message);
        self.is_error = true;
    }

    pub fn run_time_error(&mut self, operator: &Token, message: String){
        println!("{} \n[line {}]", message, operator.line);
        self.is_error = true;
    }
}


