use gpui::{
    AnyElement, IntoElement, ParentElement, RenderOnce, SharedString, Styled, div,
    prelude::FluentBuilder, px, rems,
};
use gpui_component::ActiveTheme;

#[derive(IntoElement)]
pub struct Variant {
    pub name: SharedString,
    pub element: AnyElement,
}

impl Variant {
    pub fn new(name: impl Into<SharedString>, element: AnyElement) -> Self {
        Self {
            name: name.into(),
            element,
        }
    }
}

impl RenderOnce for Variant {
    fn render(self, window: &mut gpui::Window, cx: &mut gpui::App) -> impl gpui::IntoElement {
        let theme = cx.theme();

        div()
            .pt_2()
            .flex()
            .flex_col()
            .gap_3()
            .child(div().child(self.name.clone()).text_size(rems(1.0)))
            .child(
                div()
                    .min_h(px(100.0))
                    .w_full()
                    .p_8()
                    .flex()
                    .items_center()
                    .justify_center()
                    .border_1()
                    .border_color(theme.border)
                    .child(self.element),
            )
    }
}

#[derive(IntoElement)]
pub struct VariantGroup {
    pub title: Option<SharedString>,
    pub examples: Vec<Variant>,
}

impl VariantGroup {
    pub fn new(examples: Vec<Variant>) -> Self {
        Self {
            title: None,
            examples,
        }
    }

    pub fn with_title(title: impl Into<SharedString>, examples: Vec<Variant>) -> Self {
        Self {
            title: Some(title.into()),
            examples,
        }
    }
}

impl RenderOnce for VariantGroup {
    fn render(self, window: &mut gpui::Window, cx: &mut gpui::App) -> impl IntoElement {
        let theme = cx.theme();

        div()
            .flex_col()
            .text_sm()
            .when_some(self.title, |this, title| {
                this.gap_4().child(
                    div()
                        .flex()
                        .items_center()
                        .gap_3()
                        .mt_4()
                        .mb_1()
                        .child(
                            div()
                                .flex_none()
                                .text_size(px(10.))
                                .child(title.to_uppercase()),
                        )
                        .child(div().h_px().w_full().flex_1().bg(theme.border)),
                )
            })
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_start()
                    .w_full()
                    .gap_6()
                    .children(self.examples),
            )
    }
}

pub fn variant(name: impl Into<SharedString>, example: AnyElement) -> Variant {
    Variant::new(name, example)
}

pub fn group(examples: Vec<Variant>) -> VariantGroup {
    VariantGroup::new(examples)
}

pub fn titled_group(title: impl Into<SharedString>, examples: Vec<Variant>) -> VariantGroup {
    VariantGroup::with_title(title, examples)
}
