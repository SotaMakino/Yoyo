use crate::{
    css::{Unit, Value},
    style::{self, Display},
};

#[derive(Default, Debug, Clone, Copy)]
struct Dimensions {
    content: Rect,
    padding: EdgeSizes,
    border: EdgeSizes,
    margin: EdgeSizes,
}

impl Dimensions {
    // The area covered by the content area plus its padding.
    fn padding_box(self) -> Rect {
        self.content.expanded_by(self.padding)
    }
    // The area covered by the content area plus padding and borders.
    fn border_box(self) -> Rect {
        self.padding_box().expanded_by(self.border)
    }
    // The area covered by the content area plus padding, borders, and margin.
    fn margin_box(self) -> Rect {
        self.border_box().expanded_by(self.margin)
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct Rect {
    width: f32,
    height: f32,
    x: f32,
    y: f32,
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
struct EdgeSizes {
    top: f32,
    right: f32,
    bottom: f32,
    left: f32,
}

#[derive(Debug)]
pub struct LayoutBox<'a> {
    dimensions: Dimensions,
    box_type: BoxType<'a>,
    children: Vec<LayoutBox<'a>>,
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
            BoxType::InlineNode(_) => todo!(),
            BoxType::AnonymousBlock => todo!(),
        }
    }

    fn layout_block(&mut self, containing_block: &Dimensions) {
        self.calculate_block_width(&containing_block);

        self.calculate_block_position(&containing_block);

        self.layout_block_children();

        self.calculate_block_height();
    }

    fn calculate_block_width(&mut self, containing_block: &Dimensions) {
        let style = self.get_style_node();

        let auto = Value::Keyword("auto".to_string());
        let mut width = style.value("width").unwrap_or(auto.clone());

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
    }

    fn calculate_block_position(&mut self, containing_block: &Dimensions) {
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

        d.content.x = containing_block.content.x + d.margin.left + d.border.left + d.padding.left;

        // Position the box below all the previous boxes in the container.
        d.content.y = containing_block.content.height
            + containing_block.content.y
            + d.margin.top
            + d.border.top
            + d.padding.top;
    }

    fn layout_block_children(&mut self) {
        let d = &mut self.dimensions;
        for child in &mut self.children {
            child.layout(*d);
            // Track the height so each child is laid out below the previous content.
            d.content.height = d.content.height + child.dimensions.margin_box().height;
        }
    }

    fn calculate_block_height(&mut self) {
        // If the height is set to an explicit length, use that exact length.
        // Otherwise, just keep the value set by `layout_block_children`.
        if let Some(Value::Length(h, Px)) = self.get_style_node().value("height") {
            self.dimensions.content.height = h;
        }
    }
}

#[derive(Debug)]
enum BoxType<'a> {
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
