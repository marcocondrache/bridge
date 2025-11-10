use gpui::{
    AnyElement, App, ClickEvent, CursorStyle, Div, ElementId, FocusHandle, InteractiveElement,
    IntoElement, MouseButton, ParentElement, RenderOnce, StatefulInteractiveElement, Styled,
    Window, div, prelude::FluentBuilder, px,
};
use smallvec::SmallVec;

use crate::traits::{clickable::Clickable, disableable::Disableable, styled_ext::StyledExt};

#[derive(IntoElement)]
pub(crate) struct ButtonBase {
    id: ElementId,
    base: Div,
    disabled: bool,
    cursor_style: CursorStyle,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    children: SmallVec<[AnyElement; 1]>,
    focus_handle: Option<FocusHandle>,
}

impl ButtonBase {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            base: div(),
            disabled: false,
            on_click: None,
            children: SmallVec::new(),
            cursor_style: CursorStyle::PointingHand,
            focus_handle: None,
        }
    }
}

impl RenderOnce for ButtonBase {
    fn render(self, _: &mut gpui::Window, cx: &mut gpui::App) -> impl gpui::IntoElement {
        self.base
            .h_flex()
            .id(self.id)
            .flex_none()
            .when_some(self.focus_handle, |this, focus_handle| {
                this.track_focus(&focus_handle)
            })
            .when_else(
                self.disabled,
                |this| this.cursor_not_allowed(),
                |this| this.cursor(self.cursor_style),
            )
            .when_some(self.on_click.filter(|_| !self.disabled), |this, handler| {
                this.on_mouse_down(MouseButton::Left, |_, window, _| window.prevent_default())
                    .on_click(move |event, window, cx| {
                        cx.stop_propagation();
                        (handler)(event, window, cx)
                    })
            })
            .children(self.children)
    }
}

impl ParentElement for ButtonBase {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Clickable for ButtonBase {
    fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    fn cursor_style(mut self, cursor_style: CursorStyle) -> Self {
        self.cursor_style = cursor_style;
        self
    }
}

impl Disableable for ButtonBase {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}
