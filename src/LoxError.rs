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
}

