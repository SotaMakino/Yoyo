mod css;
mod dom;
mod html;
mod layout;
mod style;

fn main() {
    let css = "
    h1,
    h2,
    h3 {
      margin: auto;
      display: inline;
    }

    p {
        color: #ffffff;
    }
    ";
    let html = "
        <h1 id='1'>Test</h1>
    ";
    let root = html::parse(html.to_string());
    let style_sheet = css::parse(css.to_string());
    let style_node = style::style_tree(&root, &style_sheet);
    println!("{:?}", layout::build_layout_tree(&style_node));
}
