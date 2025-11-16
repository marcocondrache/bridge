use gpui::{
    AnyElement, App, ClickEvent, CursorStyle, Div, ElementId, FocusHandle, InteractiveElement,
    IntoElement, MouseButton, ParentElement, RenderOnce, StatefulInteractiveElement, Styled,
    Window, div, prelude::FluentBuilder,
};
use gpui_component::ActiveTheme;
use smallvec::SmallVec;

use crate::prelude::*;

#[derive(IntoElement)]
pub(crate) struct ButtonBase {
    id: ElementId,
    base: Div,
    pub(crate) size: Size,
    pub(crate) layout: Layout,
    pub(crate) semantic: Semantic,
    disabled: bool,
    active: bool,
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
            size: Size::default(),
            semantic: Semantic::default(),
            layout: Layout::default(),
            disabled: false,
            active: false,
            on_click: None,
            children: SmallVec::new(),
            cursor_style: CursorStyle::PointingHand,
            focus_handle: None,
        }
    }
}

impl RenderOnce for ButtonBase {
    fn render(self, _: &mut gpui::Window, cx: &mut gpui::App) -> impl gpui::IntoElement {
        let theme = cx.theme();

        let base_bg = self.semantic.background(cx);
        let base_fg = self.semantic.foreground(cx);
        let base_border = self.semantic.border(cx);
        let hover_bg = self.semantic.hover_background(cx);

        let height = self.size.height();
        let padding_x = self.size.padding_x();
        let padding_y = self.size.padding_y();
        let gap = self.size.gap();

        self.base
            .h_flex()
            .id(self.id)
            .items_center()
            .justify_center()
            .gap(gap)
            .h(height)
            .px(padding_x)
            .py(padding_y)
            .bg(base_bg)
            .text_color(base_fg)
            .rounded(theme.radius)
            .when_some(base_border, |this, color| {
                this.border_1().border_color(color)
            })
            .when(self.layout == Layout::Block, |this| this.w_full())
            .when(self.active, |this| this.bg(hover_bg))
            .when_else(
                self.disabled,
                |this| this.opacity(0.5).cursor(CursorStyle::OperationNotAllowed),
                |this| {
                    this.cursor(self.cursor_style)
                        .hover(|style| style.bg(hover_bg))
                },
            )
            .when_some(self.focus_handle, |this, focus_handle| {
                this.track_focus(&focus_handle)
                    .focus(|style| style.border_1().border_color(theme.primary))
            })
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

impl Sizable for ButtonBase {
    fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
}

impl SemanticColor for ButtonBase {
    fn semantic_variant(mut self, variant: Semantic) -> Self {
        self.semantic = variant;
        self
    }
}

impl Layoutable for ButtonBase {
    fn layout_variant(mut self, variant: Layout) -> Self {
        self.layout = variant;
        self
    }
}
