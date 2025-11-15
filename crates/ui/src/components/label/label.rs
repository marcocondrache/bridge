use gpui::{
    Div, FontWeight, IntoElement, ParentElement, RenderOnce, SharedString, Styled, div,
    prelude::FluentBuilder,
};

use crate::prelude::*;

#[derive(IntoElement)]
pub struct Label {
    base: Div,
    label: SharedString,
    semantic: Semantic,
    weight: Option<FontWeight>,
    truncate: bool,
}

impl Label {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            base: div(),
            label: label.into(),
            semantic: Semantic::default(),
            weight: None,
            truncate: false,
        }
    }

    pub fn set_text(&mut self, text: impl Into<SharedString>) {
        self.label = text.into();
    }

    pub fn truncate(mut self, truncate: bool) -> Self {
        self.truncate = truncate;
        self
    }
}

impl RenderOnce for Label {
    fn render(self, _: &mut gpui::Window, cx: &mut gpui::App) -> impl gpui::IntoElement {
        let foreground = self.semantic.foreground(cx);

        self.base
            .text_color(foreground)
            .when_some(self.weight, |this, weight| this.font_weight(weight))
            .when(self.truncate, |this| {
                this.overflow_x_hidden().text_ellipsis()
            })
            .child(self.label)
    }
}

impl SemanticColor for Label {
    fn semantic_variant(mut self, variant: crate::prelude::Semantic) -> Self {
        self.semantic = variant;
        self
    }
}
