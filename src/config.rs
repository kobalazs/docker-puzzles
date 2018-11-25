pub struct Config {
    pub directory: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let directory = match args.next() {
            Some(directory) => directory,
            None => return Err("Didn't get a directory string"),
        };

        Ok(Config { directory })
    }
}
