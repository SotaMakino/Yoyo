use cursive::{
    view::{IntoBoxedView, View, ViewWrapper},
    views::{DummyView, LinearLayout, Panel, TextView},
};

use crate::{
    dom::{Node, NodeType},
    layout::{BoxType, LayoutBox},
    style::StyledNode,
};

pub type ElementContainer = Box<dyn View>;

pub fn new_element_container() -> ElementContainer {
    (DummyView {}).into_boxed_view()
}

pub fn to_element_container(layout: LayoutBox) -> ElementContainer {
    match layout.box_type {
        BoxType::BlockNode(style_node) | BoxType::InlineNode(style_node) => match style_node {
            StyledNode {
                node:
                    Node {
                        node_type: NodeType::Element(ref element),
                        ..
                    },
                ..
            } => {
                let mut panel =
                    Panel::new(LinearLayout::vertical()).title(element.tag_name.clone());
                // element.tag_name.as_str();
                for child in layout.children.into_iter() {
                    panel.with_view_mut(|v| v.add_child(to_element_container(child)));
                }

                panel.into_boxed_view()
            }
            StyledNode {
                node:
                    Node {
                        node_type: NodeType::Text(ref text),
                        ..
                    },
                ..
            } => {
                let text_to_display = text.clone();
                let text_to_display = text_to_display.replace("\n", "");
                let text_to_display = text_to_display.trim();
                if !text_to_display.is_empty() {
                    TextView::new(text_to_display).into_boxed_view()
                } else {
                    (DummyView {}).into_boxed_view()
                }
            }
            _ => (DummyView {}).into_boxed_view(),
        },
        BoxType::AnonymousBlock => {
            let mut p = Panel::new(LinearLayout::horizontal());
            for child in layout.children.into_iter() {
                p.with_view_mut(|v| v.add_child(to_element_container(child)));
            }

            p.into_boxed_view()
        }
    }
}
