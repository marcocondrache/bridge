use gpui::{
    ElementId, IntoElement, ParentElement, RenderOnce, SharedString, Styled, div,
    prelude::FluentBuilder,
};
use gpui_component::h_flex;
use ui_component::{Component, titled_group, variant};
use ui_macros::RegisterComponent;

use crate::prelude::*;

use crate::components::{
    button::ButtonBase,
    label::{Label, SpinnerLabel},
};
use crate::traits::Toggleable;

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
}

impl RenderOnce for Button {
    fn render(self, _: &mut gpui::Window, cx: &mut gpui::App) -> impl IntoElement {
        let size = self.base.size;
        let text_color = self.base.semantic.foreground(cx);

        self.base.child(
            h_flex()
                .items_center()
                .when_some(self.label.filter(|_| !self.loading), |parent, label| {
                    parent.child(Label::new(label).size(size).color(text_color))
                })
                .when(self.loading, |this| {
                    this.child(SpinnerLabel::new().size(size).color(text_color))
                }),
        )
    }
}

impl Disableable for Button {
    fn disabled(mut self, disabled: bool) -> Self {
        self.base = self.base.disabled(disabled);
        self
    }
}

impl Toggleable for Button {
    fn toggle_state(self, _selected: bool) -> Self {
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

impl Loadable for Button {
    fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }
}

impl Sizable for Button {
    fn size(mut self, size: Size) -> Self {
        self.base = self.base.size(size);
        self
    }
}

impl SemanticColor for Button {
    fn semantic_variant(mut self, variant: Semantic) -> Self {
        self.base = self.base.semantic_variant(variant);
        self
    }
}

impl Layoutable for Button {
    fn layout_variant(mut self, variant: Layout) -> Self {
        self.base = self.base.layout_variant(variant);
        self
    }
}

impl Component for Button {
    fn showcase(_window: &mut gpui::Window, _cx: &mut gpui::App) -> Option<gpui::AnyElement> {
        Some(
            div()
                .v_flex()
                .gap_6()
                .children(vec![
                    titled_group(
                        "Semantic Variants",
                        vec![
                            variant(
                                "Default",
                                Button::new("default").label("Default").into_any_element(),
                            ),
                            variant(
                                "Primary",
                                Button::new("primary")
                                    .label("Primary")
                                    .primary()
                                    .into_any_element(),
                            ),
                            variant(
                                "Secondary",
                                Button::new("secondary")
                                    .label("Secondary")
                                    .secondary()
                                    .into_any_element(),
                            ),
                            variant(
                                "Destructive",
                                Button::new("destructive")
                                    .label("Destructive")
                                    .destructive()
                                    .into_any_element(),
                            ),
                            variant(
                                "Ghost",
                                Button::new("ghost")
                                    .label("Ghost")
                                    .ghost()
                                    .into_any_element(),
                            ),
                            variant(
                                "Outline",
                                Button::new("outline")
                                    .label("Outline")
                                    .outline()
                                    .into_any_element(),
                            ),
                        ],
                    ),
                    titled_group(
                        "Sizes",
                        vec![
                            variant(
                                "Small",
                                Button::new("small")
                                    .label("Small")
                                    .small()
                                    .primary()
                                    .into_any_element(),
                            ),
                            variant(
                                "Default",
                                Button::new("default-size")
                                    .label("Default")
                                    .primary()
                                    .into_any_element(),
                            ),
                            variant(
                                "Medium",
                                Button::new("medium")
                                    .label("Medium")
                                    .medium()
                                    .primary()
                                    .into_any_element(),
                            ),
                            variant(
                                "Large",
                                Button::new("large")
                                    .label("Large")
                                    .large()
                                    .primary()
                                    .into_any_element(),
                            ),
                        ],
                    ),
                    titled_group(
                        "States",
                        vec![
                            variant(
                                "Default",
                                Button::new("state-default")
                                    .label("Default")
                                    .primary()
                                    .into_any_element(),
                            ),
                            variant(
                                "Loading",
                                Button::new("state-loading")
                                    .label("Loading")
                                    .primary()
                                    .loading(true)
                                    .into_any_element(),
                            ),
                            variant(
                                "Disabled",
                                Button::new("state-disabled")
                                    .label("Disabled")
                                    .primary()
                                    .disabled(true)
                                    .into_any_element(),
                            ),
                        ],
                    ),
                    titled_group(
                        "Layout Variants",
                        vec![
                            variant(
                                "Standalone",
                                Button::new("layout-standalone")
                                    .label("Standalone")
                                    .primary()
                                    .standalone()
                                    .into_any_element(),
                            ),
                            variant(
                                "Block",
                                Button::new("layout-block")
                                    .label("Block (Full Width)")
                                    .primary()
                                    .block()
                                    .into_any_element(),
                            ),
                        ],
                    ),
                ])
                .into_any_element(),
        )
    }
}
