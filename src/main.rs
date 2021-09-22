mod dom;
mod html;

fn main() {
    let source = "
    <html>
        <head>
            <title id='1'>Test</title>
        </head>
        <body>
            <!--  comment text. -->
            <p class='inner'>Hello, <span id='name'>world!</span></p>
        </body>
    </html>
    ";

    println!("{:?}", html::parse(source.to_string()));
}
