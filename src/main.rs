mod dom;
mod html;

fn main() {
    let source = "
    <html>
        <head>
            <title id='1'>Test</title>
            <0></0>
        </head>
    </html>
    ";

    println!("{:?}", html::parse(source.to_string()));
}
