use crate::error::UserError;
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    pub directory: String,
}

impl Config {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(args: &[String]) -> Result<Self, Box<dyn Error>> {
        match args.get(0) {
            Some(arg) => Ok(Self {
                directory: arg.to_string(),
            }),
            None => Err(UserError::boxed("Didn't get a directory argument")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_config_if_arg_found() {
        let args = vec![String::from("test")];
        let config = Config::new(&args).unwrap();
        assert_eq!("test", config.directory);
    }

    #[test]
    fn it_returns_error_if_arg_not_found() {
        let args = vec![];
        let error = Config::new(&args).expect_err("Error expected");
        assert_eq!("Didn't get a directory argument", error.to_string());
    }
}
