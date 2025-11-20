use gpui::{
    App, ClickEvent, CursorStyle, Div, ElementId, InteractiveElement, IntoElement, MouseButton,
    RenderOnce, StatefulInteractiveElement, Window, div,
};
use gpui_component::{ActiveTheme, Icon, IconName};

use crate::{prelude::*, traits::Toggleable};

#[derive(IntoElement)]
pub struct Checkbox {
    id: ElementId,
    base: Div,
    size: Size,
    selected: bool,
    disabled: bool,
    cursor_style: CursorStyle,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl Checkbox {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            base: div(),
            size: Size::default(),
            selected: false,
            disabled: false,
            on_click: None,
            cursor_style: CursorStyle::PointingHand,
        }
    }
}

impl Clickable for Checkbox {
    fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    fn cursor_style(mut self, cursor_style: gpui::CursorStyle) -> Self {
        self.cursor_style = cursor_style;
        self
    }
}

impl Toggleable for Checkbox {
    fn toggle_state(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl Sizable for Checkbox {
    fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
}

impl Disableable for Checkbox {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl RenderOnce for Checkbox {
    fn render(self, _window: &mut gpui::Window, cx: &mut gpui::App) -> impl IntoElement {
        let theme = cx.theme();
        let heigth = self.size.height();
        let text_size = self.size.font_size();

        self.base
            .id(self.id)
            .size(heigth)
            .v_flex()
            .items_center()
            .justify_center()
            .border_1()
            .border_color(theme.border)
            .cursor(self.cursor_style)
            .text_color(theme.primary_foreground)
            .text_size(text_size)
            .bg(theme.input)
            .when(self.selected && !self.disabled, |this| {
                this.bg(theme.primary).child(Icon::new(IconName::Check))
            })
            .when(self.disabled, |this| {
                this.cursor(CursorStyle::OperationNotAllowed)
                    .bg(theme.primary.opacity(0.5))
            })
            .when_some(self.on_click.filter(|_| !self.disabled), |this, handler| {
                this.on_mouse_down(MouseButton::Left, |_, window, _| window.prevent_default())
                    .on_click(move |event, window, cx| {
                        cx.stop_propagation();
                        (handler)(event, window, cx)
                    })
            })
    }
}
