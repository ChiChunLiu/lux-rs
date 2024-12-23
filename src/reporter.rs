pub trait Reporter {
    fn error(&mut self, line: usize, message: &str);
    fn report(&mut self, line: usize, error_where: &str, message: &str);
}

#[derive(Default)]
pub struct StdoutReporter {
    had_error: bool,
}

impl Reporter for StdoutReporter {
    fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: usize, error_where: &str, message: &str) {
        println!("[line {}] Error {}: {}", line, error_where, message);
        self.had_error = true;
    }
}
