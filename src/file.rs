use std::env;
use std::fs::File;
use std::io::Read;

pub struct Config {
    pub html_filename: String,
    pub css_filename: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let html_filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a html file path"),
        };
        let css_filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a css file path"),
        };

        Ok(Config {
            html_filename,
            css_filename,
        })
    }
}

pub fn read_source(filename: String) -> String {
    let mut str = String::new();
    File::open(filename)
        .unwrap()
        .read_to_string(&mut str)
        .unwrap();
    str
}
