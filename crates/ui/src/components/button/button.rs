use gpui::{
    ElementId, IntoElement, ParentElement, RenderOnce, SharedString, Styled, prelude::FluentBuilder,
};
use gpui_component::h_flex;

use crate::{
    components::{
        button::ButtonBase,
        label::{Label, SpinnerLabel},
    },
    traits::{clickable::Clickable, disableable::Disableable},
};

#[derive(IntoElement)]
pub struct Button {
    base: ButtonBase,
    label: Option<SharedString>,
    loading: bool,
}

impl Button {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: ButtonBase::new(id),
            label: None,
            loading: false,
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }
}

impl RenderOnce for Button {
    fn render(self, _: &mut gpui::Window, cx: &mut gpui::App) -> impl IntoElement {
        self.base.child(
            h_flex()
                .size_full()
                .items_center()
                .justify_center()
                .when_some(self.label.filter(|_| !self.loading), |parent, label| {
                    parent.child(Label::new(label))
                })
                .when(self.loading, |this| this.child(SpinnerLabel::new())),
        )
    }
}

impl Disableable for Button {
    fn disabled(mut self, disabled: bool) -> Self {
        self.base = self.base.disabled(disabled);
        self
    }
}

impl Clickable for Button {
    fn on_click(
        mut self,
        handler: impl Fn(&gpui::ClickEvent, &mut gpui::Window, &mut gpui::App) + 'static,
    ) -> Self {
        self.base = self.base.on_click(handler);
        self
    }

    fn cursor_style(mut self, cursor_style: gpui::CursorStyle) -> Self {
        self.base = self.base.cursor_style(cursor_style);
        self
    }
}
