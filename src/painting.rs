use std::iter::repeat;

use crate::{css, layout};

type DisplayList = Vec<DisplayCommand>;

#[derive(Debug)]
pub enum DisplayCommand {
    SolidColor(css::Color, layout::Rect),
    Text(String, css::Color, layout::Rect),
}

pub fn build_display_list(layout_root: &layout::LayoutBox) -> DisplayList {
    let mut list = Vec::new();
    render_layout_box(&mut list, layout_root);
    list
}

fn render_layout_box(list: &mut DisplayList, layout_box: &layout::LayoutBox) {
    render_background(list, layout_box);
    render_borders(list, layout_box);
    render_text(list, layout_box);

    for child in &layout_box.children {
        render_layout_box(list, child);
    }
}

fn render_text(list: &mut DisplayList, layout_box: &layout::LayoutBox) {
    if let Some(text) = get_text(layout_box) {
        println!("i got it. {:?}", text);
        list.push(DisplayCommand::Text(
            text,
            css::Color {
                r: 129,
                g: 45,
                b: 211,
                a: 255,
            },
            layout_box.dimensions.border_box(),
        ))
    }
}

fn get_text(layout_box: &layout::LayoutBox) -> Option<String> {
    match layout_box.box_type {
        layout::BoxType::InlineNode(style) => style.text(),
        _ => None,
    }
}

fn render_background(list: &mut DisplayList, layout_box: &layout::LayoutBox) {
    if let Some(color) = get_color(layout_box, "background") {
        list.push(DisplayCommand::SolidColor(
            color,
            layout_box.dimensions.border_box(),
        ))
    }
}

fn get_color(layout_box: &layout::LayoutBox, name: &str) -> Option<css::Color> {
    match layout_box.box_type {
        layout::BoxType::BlockNode(style) | layout::BoxType::InlineNode(style) => {
            match style.value(name) {
                Some(css::Value::Color(color)) => Some(color),
                _ => None,
            }
        }
        layout::BoxType::AnonymousBlock => None,
    }
}

fn render_borders(list: &mut DisplayList, layout_box: &layout::LayoutBox) {
    let color = match get_color(layout_box, "border-color") {
        Some(color) => color,
        _ => return,
    };

    let d = &layout_box.dimensions;
    let border_box = d.border_box();

    // Left border
    list.push(DisplayCommand::SolidColor(
        color,
        layout::Rect {
            x: border_box.x,
            y: border_box.y,
            width: d.border.left,
            height: border_box.height,
        },
    ));

    // Right border
    list.push(DisplayCommand::SolidColor(
        color,
        layout::Rect {
            x: border_box.x + border_box.width - d.border.right,
            y: border_box.y,
            width: d.border.right,
            height: border_box.height,
        },
    ));

    // Top border
    list.push(DisplayCommand::SolidColor(
        color,
        layout::Rect {
            x: border_box.x,
            y: border_box.y,
            width: border_box.width,
            height: d.border.top,
        },
    ));

    // Bottom border
    list.push(DisplayCommand::SolidColor(
        color,
        layout::Rect {
            x: border_box.x,
            y: border_box.y + border_box.height - d.border.bottom,
            width: border_box.width,
            height: d.border.bottom,
        },
    ));
}

/// Paint a tree of LayoutBoxes to an array of pixels.
pub fn paint(layout_root: &layout::LayoutBox, bounds: layout::Rect) -> Canvas {
    let display_list = build_display_list(layout_root);
    let mut canvas = Canvas::new(bounds.width as usize, bounds.height as usize);
    for item in display_list {
        canvas.paint_item(&item);
    }
    canvas
}

pub struct Canvas {
    pub pixels: Vec<css::Color>,
    pub width: usize,
    pub height: usize,
}

impl Canvas {
    // Create a blank canvas
    fn new(width: usize, height: usize) -> Canvas {
        let white = css::Color {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        };
        Canvas {
            pixels: repeat(white).take(width * height).collect(),
            width,
            height,
        }
    }

    fn paint_item(&mut self, item: &DisplayCommand) {
        match item {
            DisplayCommand::SolidColor(color, rect) => {
                // Clip the rectangle to the canvas boundaries.
                let x0 = rect.x.clamp(0.0, self.width as f32) as usize;
                let y0 = rect.y.clamp(0.0, self.height as f32) as usize;
                let x1 = (rect.x + rect.width).clamp(0.0, self.width as f32) as usize;
                let y1 = (rect.y + rect.height).clamp(0.0, self.height as f32) as usize;

                for y in y0..y1 {
                    for x in x0..x1 {
                        // TODO: alpha compositing with existing pixel
                        self.pixels[x + y * self.width] = *color;
                    }
                }
            }
            DisplayCommand::Text(text, color, rect) => {
                let x0 = rect.x.clamp(0.0, self.width as f32) as usize;
                let y0 = rect.y.clamp(0.0, self.height as f32) as usize;
                let x1 = (rect.x + rect.width).clamp(0.0, self.width as f32) as usize;
                let y1 = (rect.y + rect.height).clamp(0.0, self.height as f32) as usize;

                for y in y0..y1 {
                    for x in x0..x1 {
                        // TODO: alpha compositing with existing pixel
                        self.pixels[x + y * self.width] = *color;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{html, style};

    use super::*;

    #[test]
    fn test_build_display_list() {
        let css = "
        div {
          width: 150px;
          height: 50px;
          background: #00ccff;
        }
        ";
        let html = "
            <div><div></div></div>
        ";
        let root = html::parse(html.to_string());
        let style_sheet = css::parse(css.to_string());
        let style_node = style::style_tree(&root, &style_sheet);
        let layout_box = layout::build_layout_tree(&style_node);
        println!("{:?}", build_display_list(&layout_box));
    }
}
