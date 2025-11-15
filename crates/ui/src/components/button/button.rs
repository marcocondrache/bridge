use gpui::{
    ElementId, IntoElement, ParentElement, RenderOnce, SharedString, Styled, div,
    prelude::FluentBuilder,
};
use gpui_component::h_flex;
use ui_component::{Component, titled_group, variant};
use ui_macros::RegisterComponent;

use crate::{
    components::{
        button::ButtonBase,
        label::{Label, SpinnerLabel},
    },
    styles::{Sizable, Size},
    traits::{clickable::Clickable, disableable::Disableable, styled_ext::StyledExt},
};

#[derive(IntoElement, RegisterComponent)]
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
                .items_center()
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

impl Sizable for Button {
    fn with_size(mut self, size: Size) -> Self {
        self.base = self.base.with_size(size);
        self
    }
}

impl Component for Button {
    fn showcase(_window: &mut gpui::Window, _cx: &mut gpui::App) -> Option<gpui::AnyElement> {
        Some(
            div()
                .v_flex()
                .gap_6()
                .children(vec![titled_group(
                    "Button",
                    vec![variant(
                        "Default",
                        Button::new("default").label("Example").into_any_element(),
                    )],
                )])
                .into_any_element(),
        )
    }
}
