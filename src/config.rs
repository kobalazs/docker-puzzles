#[derive(Debug)]
pub struct Config {
    pub directory: String,
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, &'static str> {
        match args.get(0) {
            Some(arg) => Ok(Config { directory: arg.to_string() }),
            None => Err("Didn't get a directory argument")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_config_if_arg_found() {
        let args = vec![String::from("test")];
        let config = Config::new(args).unwrap();
        assert_eq!("test", config.directory);
    }

    #[test]
    fn it_returns_error_if_arg_not_found() {
        let args = vec![];
        let error = Config::new(args).expect_err("Error expected");
        assert_eq!("Didn't get a directory argument", error);
    }
}
