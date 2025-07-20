pub struct ErrorHandler {
    had_error: bool,
}

impl ErrorHandler {
    pub fn new() -> Self {
        ErrorHandler { had_error: false }
    }

    pub fn log_error(&mut self, line: usize, pos: usize, message: &str) {
        self.had_error = true;
        eprintln!("[line {line}, character {pos}] Error : {message}")
    }
}
