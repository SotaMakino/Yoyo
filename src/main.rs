mod css;
mod dom;
mod html;
mod style;

fn main() {
    let source = "
            <title id='1'>Test</title>
    ";

    println!("{:?}", html::parse(source.to_string()));
}
