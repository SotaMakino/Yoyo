use std::{fs::File, io::BufWriter};

pub mod css;
pub mod dom;
pub mod file;
pub mod html;
pub mod layout;
pub mod painting;
pub mod pdf;
pub mod style;

pub fn run(config: file::Config) -> bool {
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

    pdf::render(&layout_root, viewport.content, &mut file).is_ok()
}
