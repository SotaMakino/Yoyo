use std::env;
use std::{fs::File, io::BufWriter};

mod css;
mod dom;
mod file;
mod html;
mod layout;
mod painting;
mod pdf;
mod style;

fn main() {
    let config = file::Config::new(env::args()).unwrap();
    let html = file::read_source(config.html_filename);
    let css = file::read_source(config.css_filename);

    let mut viewport: layout::Dimensions = Default::default();
    viewport.content.width = 800.0;
    viewport.content.height = 600.0;

    let root_node = html::parse(html);
    let stylesheet = css::parse(css);
    let style_root = style::style_tree(&root_node, &stylesheet);
    let layout_root = layout::layout_tree(&style_root, viewport);

    let filename = "output.pdf";
    let mut file = BufWriter::new(File::create(&filename).unwrap());

    let is_rendered = { pdf::render(&layout_root, viewport.content, &mut file).is_ok() };

    if is_rendered {
        println!("Saved output as {}", filename)
    } else {
        println!("Error saving output as {}", filename)
    }
}
