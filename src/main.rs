use std::env;

use yoyo::file;

fn main() {
    let config = file::Config::new(env::args()).unwrap();
    let is_rendered = yoyo::run(config);

    if is_rendered {
        println!("Saved output")
    } else {
        println!("Error saving output")
    }
}
