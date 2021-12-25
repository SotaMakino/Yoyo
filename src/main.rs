use std::env;
use yoyo::file;

fn main() {
    let config = file::Config::new(env::args()).unwrap();

    yoyo::run(config);
}
