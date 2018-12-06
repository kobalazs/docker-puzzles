use std::error::Error;

#[derive(Debug, Clone)]
pub struct UserError<'a> {
    message: &'a str,
}

impl<'a> UserError<'a> {
    pub fn new(message: &'a str) -> UserError<'a> {
        UserError { message }
    }

    pub fn boxed(message: &'a str) -> Box<UserError<'a>> {
        Box::new(UserError::new(message))
    }
}

impl<'a> std::fmt::Display for UserError<'a> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{}", self.message)
    }
}

impl<'a> Error for UserError<'a> {
    fn description(&self) -> &str {
        &self.message
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
