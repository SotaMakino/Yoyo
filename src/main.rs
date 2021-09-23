mod css;
mod dom;
mod html;

fn main() {
    let source = "
            <title id='1'>Test</title>
    ";

    println!("{:?}", html::parse(source.to_string()));
}
