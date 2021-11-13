use crate::{
    css::{Unit, Value},
    style::{self, Display},
};

#[derive(Default, Debug, Clone, Copy)]
pub struct Dimensions {
    pub content: Rect,
    pub padding: EdgeSizes,
    pub border: EdgeSizes,
    pub margin: EdgeSizes,
}

impl Dimensions {
    // The area covered by the content area plus its padding.
    pub fn padding_box(self) -> Rect {
        self.content.expanded_by(self.padding)
    }
    // The area covered by the content area plus padding and borders.
    pub fn border_box(self) -> Rect {
        self.padding_box().expanded_by(self.border)
    }
    // The area covered by the content area plus padding, borders, and margin.
    pub fn margin_box(self) -> Rect {
        self.border_box().expanded_by(self.margin)
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Rect {
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
}

impl Rect {
    fn expanded_by(self, edge: EdgeSizes) -> Rect {
        Rect {
            x: self.x - edge.left,
            y: self.y - edge.top,
            width: self.width + edge.left + edge.right,
            height: self.height + edge.top + edge.bottom,
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct EdgeSizes {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

#[derive(Debug)]
pub struct LayoutBox<'a> {
    pub dimensions: Dimensions,
    pub box_type: BoxType<'a>,
    pub children: Vec<LayoutBox<'a>>,
}

impl<'a> LayoutBox<'a> {
    fn new(box_type: BoxType) -> LayoutBox {
        LayoutBox {
            dimensions: Default::default(),
            box_type,
            children: Vec::new(),
        }
    }

    fn get_style_node(&mut self) -> &'a style::StyledNode<'a> {
        match self.box_type {
            BoxType::BlockNode(node) | BoxType::InlineNode(node) => node,
            BoxType::AnonymousBlock => panic!("Anonymous block box has no style node"),
        }
    }

    fn get_inline_container(&mut self) -> &mut LayoutBox<'a> {
        match self.box_type {
            BoxType::InlineNode(_) | BoxType::AnonymousBlock => self,
            BoxType::BlockNode(_) => {
                match self.children.last() {
                    Some(&LayoutBox {
                        box_type: BoxType::AnonymousBlock,
                        ..
                    }) => {}
                    _ => self.children.push(LayoutBox::new(BoxType::AnonymousBlock)),
                }
                self.children.last_mut().unwrap()
            }
        }
    }

    fn layout(&mut self, containing_block: Dimensions) {
        match self.box_type {
            BoxType::BlockNode(_) => self.layout_block(&containing_block),
            BoxType::InlineNode(_) => self.layout_inline(&containing_block),
            BoxType::AnonymousBlock => self.layout_anonymous_block(&containing_block),
        }
    }

    fn layout_block(&mut self, containing_block: &Dimensions) {
        println!("its block");
        self.calculate_block_width(containing_block);

        self.calculate_position_by_styles();
        self.calculate_block_position(containing_block);

        self.layout_block_children();

        self.calculate_block_height();
    }

    fn layout_anonymous_block(&mut self, containing_block: &Dimensions) {
        println!("its anonymous");

        self.calculate_anonymous_position(containing_block);

        self.layout_inline_children(containing_block);

        let d = &mut self.dimensions;
        d.content.height = containing_block.content.height;
    }

    fn layout_inline(&mut self, containing_block: &Dimensions) {
        println!("its inline");
        self.calculate_position_by_styles();
        self.calculate_inline_position(containing_block);

        self.calculate_inline_width();

        self.calculate_block_height();

        self.layout_inline_children(containing_block);
    }

    fn calculate_block_width(&mut self, containing_block: &Dimensions) {
        let style = self.get_style_node();

        let auto = Value::Keyword("auto".to_string());
        let mut width = style.value("width").unwrap_or_else(|| auto.clone());

        let zero = Value::Length(0.0, Unit::Px);

        let mut margin_left = style.lookup("margin-left", "margin", &zero);
        let mut margin_right = style.lookup("margin-right", "margin", &zero);

        let border_left = style.lookup("border-left-width", "border-width", &zero);
        let border_right = style.lookup("border-right-width", "border-width", &zero);

        let padding_left = style.lookup("padding-left", "padding", &zero);
        let padding_right = style.lookup("padding-right", "padding", &zero);

        let total: f32 = [
            &margin_left,
            &margin_right,
            &border_left,
            &border_right,
            &padding_left,
            &padding_right,
            &width,
        ]
        .iter()
        .map(|v| v.to_px())
        .sum();

        if width != auto && total > containing_block.content.width {
            if margin_left == auto {
                margin_left = Value::Length(0.0, Unit::Px);
            }
            if margin_right == auto {
                margin_right = Value::Length(0.0, Unit::Px);
            }
        }

        let underflow = containing_block.content.width - total;

        match (width == auto, margin_left == auto, margin_right == auto) {
            (false, false, false) => {
                margin_right = Value::Length(margin_right.to_px() + underflow, Unit::Px);
            }

            (false, false, true) => {
                margin_right = Value::Length(underflow, Unit::Px);
            }
            (false, true, false) => {
                margin_left = Value::Length(underflow, Unit::Px);
            }

            (true, _, _) => {
                if margin_left == auto {
                    margin_left = Value::Length(0.0, Unit::Px);
                }
                if margin_right == auto {
                    margin_right = Value::Length(0.0, Unit::Px);
                }

                if underflow >= 0.0 {
                    width = Value::Length(underflow, Unit::Px);
                } else {
                    width = Value::Length(0.0, Unit::Px);
                    margin_right = Value::Length(margin_right.to_px() + underflow, Unit::Px);
                }
            }

            (false, true, true) => {
                let half_of_underflow = underflow / 2.0;
                margin_left = Value::Length(half_of_underflow, Unit::Px);
                margin_right = Value::Length(half_of_underflow, Unit::Px);
            }
        }

        let d = &mut self.dimensions;
        d.content.width = width.to_px();

        d.padding.left = padding_left.to_px();
        d.padding.right = padding_right.to_px();

        d.border.left = border_left.to_px();
        d.border.right = border_right.to_px();

        d.margin.left = margin_left.to_px();
        d.margin.right = margin_right.to_px();
    }

    fn calculate_inline_width(&mut self) {
        let style = self.get_style_node();
        let zero = Value::Length(0.0, Unit::Px);
        let width = style.value("width").unwrap_or(zero);
        let zero = Value::Length(0.0, Unit::Px);
        let margin_left = style.lookup("margin-left", "margin", &zero);
        let margin_right = style.lookup("margin-right", "margin", &zero);

        let border_left = style.lookup("border-left-width", "border-width", &zero);
        let border_right = style.lookup("border-right-width", "border-width", &zero);

        let padding_left = style.lookup("padding-left", "padding", &zero);
        let padding_right = style.lookup("padding-right", "padding", &zero);

        let d = &mut self.dimensions;
        d.content.width = width.to_px();

        d.padding.left = padding_left.to_px();
        d.padding.right = padding_right.to_px();

        d.border.left = border_left.to_px();
        d.border.right = border_right.to_px();

        d.margin.left = margin_left.to_px();
        d.margin.right = margin_right.to_px();
    }

    fn calculate_position_by_styles(&mut self) {
        let style = self.get_style_node();
        let d = &mut self.dimensions;
        let zero = Value::Length(0.0, Unit::Px);

        // If margin-top or margin-bottom is `auto`, the used value is zero.
        d.margin.top = style.lookup("margin-top", "margin", &zero).to_px();
        d.margin.bottom = style.lookup("margin-bottom", "margin", &zero).to_px();

        d.border.top = style
            .lookup("border-top-width", "border-width", &zero)
            .to_px();
        d.border.bottom = style
            .lookup("border-bottom-width", "border-width", &zero)
            .to_px();

        d.padding.top = style.lookup("padding-top", "padding", &zero).to_px();
        d.padding.bottom = style.lookup("padding-bottom", "padding", &zero).to_px();
    }

    fn calculate_block_position(&mut self, containing_block: &Dimensions) {
        let d = &mut self.dimensions;
        d.content.x = containing_block.content.x + d.margin.left + d.border.left + d.padding.left;
        d.content.y = containing_block.content.height
            + containing_block.content.y
            + d.margin.top
            + d.border.top
            + d.padding.top;
    }

    fn calculate_anonymous_position(&mut self, containing_block: &Dimensions) {
        let d = &mut self.dimensions;
        d.content.x = containing_block.content.x;
        d.content.y = containing_block.content.height + containing_block.content.y
    }

    fn calculate_inline_position(&mut self, containing_block: &Dimensions) {
        let d = &mut self.dimensions;
        d.content.x = containing_block.content.x
            + containing_block.content.width
            + d.margin.left
            + d.border.left
            + d.padding.left;
        d.content.y = containing_block.content.y + d.margin.top + d.border.top + d.padding.top;
    }

    fn layout_block_children(&mut self) {
        let d = &mut self.dimensions;
        for child in &mut self.children {
            child.layout(*d);
            // Track the height so each child is laid out below the previous content.
            d.content.height += child.dimensions.margin_box().height;
        }
    }

    fn layout_inline_children(&mut self, containing_block: &Dimensions) {
        let d = &mut self.dimensions;
        for child in &mut self.children {
            println!("Target: {:?}", d.content);
            println!("Parent: {:?}", containing_block.content);
            child.layout(*d);
            let new_width = d.content.width + child.dimensions.margin_box().width;
            if new_width > containing_block.content.width {
                println!("over");
                d.content.width = 0.0;
                d.content.y += containing_block.content.y;
            } else {
                d.content.width = new_width;
            }
        }
    }

    fn calculate_block_height(&mut self) {
        // If the height is set to an explicit length, use that exact length.
        // Otherwise, just keep the value set by `layout_block_children`.
        if let Some(Value::Length(h, Unit::Px)) = self.get_style_node().value("height") {
            self.dimensions.content.height = h;
        }
    }
}

/// Transform a style tree into a layout tree.
pub fn layout_tree<'a>(
    node: &'a style::StyledNode<'a>,
    mut containing_block: Dimensions,
) -> LayoutBox<'a> {
    // The layout algorithm expects the container height to start at 0.
    // TODO: Save the initial containing block height, for calculating percent heights.
    containing_block.content.height = 0.0;

    let mut root_box = build_layout_tree(node);
    root_box.layout(containing_block);
    root_box
}

#[derive(Debug, Clone, Copy)]
pub enum BoxType<'a> {
    BlockNode(&'a style::StyledNode<'a>),
    InlineNode(&'a style::StyledNode<'a>),
    AnonymousBlock,
}

pub fn build_layout_tree<'a>(style_node: &'a style::StyledNode<'a>) -> LayoutBox<'a> {
    let mut root = LayoutBox::new(match style_node.display() {
        Display::Block => BoxType::BlockNode(style_node),
        Display::Inline => BoxType::InlineNode(style_node),
        Display::None => panic!("Root node has display: none."),
    });

    for child in &style_node.children {
        match child.display() {
            Display::Block => root.children.push(build_layout_tree(child)),
            Display::Inline => root
                .get_inline_container()
                .children
                .push(build_layout_tree(child)),
            Display::None => {}
        }
    }

    root
}

#[cfg(test)]
mod tests {
    use crate::{css, html};

    use super::*;

    #[test]
    fn test_build_layout_tree() {
        let css = "
        h1,
        h2,
        h3 {
          margin: 10px;
          width: 100px;
        }
    
        p {
            color: #ffffff;
            padding-top: 33px;
        }
        ";
        let html = "
            <h1 id='1'>Test<p>para</p></h1>
        ";
        let root = html::parse(html.to_string());
        let style_sheet = css::parse(css.to_string());
        let style_node = style::style_tree(&root, &style_sheet);
        println!("{:?}", build_layout_tree(&style_node));
    }
}
