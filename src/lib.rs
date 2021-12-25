pub mod css;
pub mod dom;
pub mod file;
pub mod html;
pub mod layout;
pub mod painting;
pub mod render;
pub mod style;

pub fn run(config: file::Config) {
    let html_source = file::read_source(config.html_filename);
    let css_source = file::read_source(config.css_filename);

    let mut viewport: layout::Dimensions = Default::default();
    viewport.content.width = 800.0;
    viewport.content.height = 600.0;

    let root_node = html::parse(html_source);
    let stylesheet = css::parse(css_source);
    let style_root = style::style_tree(&root_node, &stylesheet);
    let layout_root = layout::layout_tree(&style_root, viewport);

    let mut siv = cursive::default();
    let container = Some(render::to_element_container(layout_root));
    if let Some(c) = container {
        siv.add_fullscreen_layer(c);
    }
    siv.run();
}
