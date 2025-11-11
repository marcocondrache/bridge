use gpui::{
    App, AppContext, Axis, InteractiveElement, IntoElement, MouseButton, MouseUpEvent, Pixels,
    Render, RenderOnce, StatefulInteractiveElement, Styled, Window, deferred, div,
    prelude::FluentBuilder, px,
};

use crate::utils::placement::Placement;

#[derive(IntoElement)]
pub struct ResizeHandle<D: Clone + Render + 'static> {
    data: D,
    placement: Placement,
    on_double_click: Option<Box<dyn Fn(&mut Window, &mut App)>>,
}

impl<D> ResizeHandle<D>
where
    D: Clone + Render + 'static,
{
    pub const HANDLE_SIZE: Pixels = px(6.);

    pub fn new(data: D, placement: Placement) -> Self {
        Self {
            data,
            placement,
            on_double_click: None,
        }
    }

    pub fn on_double_click<F>(mut self, f: F) -> Self
    where
        F: Fn(&mut Window, &mut App) + 'static,
    {
        self.on_double_click = Some(Box::new(f));
        self
    }
}

impl<D: Clone + Render + 'static> RenderOnce for ResizeHandle<D> {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl gpui::IntoElement {
        let axis = self.placement.axis();

        div()
            .id("resize-handle")
            .on_mouse_down(MouseButton::Left, |_, _, cx| {
                cx.stop_propagation();
            })
            .on_drag(self.data, move |data, _, _, cx| {
                cx.stop_propagation();
                cx.new(|_| data.clone())
            })
            .when_some(self.on_double_click, |this, listener| {
                this.on_mouse_up(MouseButton::Left, move |e: &MouseUpEvent, window, cx| {
                    if e.click_count == 2 {
                        listener(window, cx);
                        cx.stop_propagation();
                    }
                })
            })
            .occlude()
            .absolute()
            .map(|handle| match axis {
                Axis::Vertical => handle.h(Self::HANDLE_SIZE).w_full().cursor_row_resize(),
                Axis::Horizontal => handle.w(Self::HANDLE_SIZE).h_full().cursor_col_resize(),
            })
            .map(|handle| match &self.placement {
                Placement::Top => handle.bottom(-Self::HANDLE_SIZE / 2.).left(px(0.)),
                Placement::Right => handle.top(px(0.)).left(-Self::HANDLE_SIZE / 2.),
                Placement::Bottom => handle.top(-Self::HANDLE_SIZE / 2.).left(px(0.)),
                Placement::Left => handle.top(px(0.)).right(-Self::HANDLE_SIZE / 2.),
            })
            .map(deferred)
    }
}
