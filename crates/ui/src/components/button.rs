use gpui::{
    AnyElement, App, ClickEvent, CursorStyle, Div, ElementId, Hsla, InteractiveElement,
    IntoElement, ParentElement, RenderOnce, Styled, Window, div,
};
use smallvec::SmallVec;

use crate::traits::{clickable::Clickable, disableable::Disableable, styled_ext::StyledExt};

#[derive(Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
}

pub struct ButtonStyle {
    background: Hsla,
    border_color: Hsla,
    label_color: Hsla,
}

#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    base: Div,
    variant: ButtonVariant,
    children: SmallVec<[AnyElement; 2]>,
    cursor_style: CursorStyle,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    is_disabled: bool,
}

impl Button {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            base: div(),
            variant: ButtonVariant::Primary,
            children: SmallVec::new(),
            on_click: None,
            is_disabled: false,
            cursor_style: CursorStyle::PointingHand,
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }
}

impl RenderOnce for Button {
    fn render(self, window: &mut Window, cx: &mut App) -> impl gpui::IntoElement {
        self.base
            .id(self.id.clone())
            .h_flex()
            .flex_shrink()
            .items_center()
            .justify_center()
            .cursor_default()
            .child(
                div()
                    .h_flex()
                    .id("label")
                    .items_center()
                    .justify_center()
                    .children(self.children),
            )
    }
}

impl ParentElement for Button {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Disableable for Button {
    fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }
}

impl Clickable for Button {
    fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    fn cursor_style(mut self, cursor_style: CursorStyle) -> Self {
        self.cursor_style = cursor_style;
        self
    }
}
