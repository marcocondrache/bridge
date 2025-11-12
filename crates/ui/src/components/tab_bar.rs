use gpui::{
    AnyElement, ElementId, InteractiveElement, ParentElement, RenderOnce,
    StatefulInteractiveElement, Styled, div,
};
use gpui_component::ActiveTheme;
use smallvec::SmallVec;

use crate::traits::styled_ext::StyledExt;

pub struct TabBar {
    id: ElementId,
    children: SmallVec<[AnyElement; 2]>,
}

impl TabBar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            children: SmallVec::new(),
        }
    }
}

impl ParentElement for TabBar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TabBar {
    fn render(self, window: &mut gpui::Window, cx: &mut gpui::App) -> impl gpui::IntoElement {
        div().id(self.id).flex().flex_none().w_full().child(
            div()
                .relative()
                .flex_1()
                .h_full()
                .overflow_hidden()
                .child(
                    div()
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .border_b_1()
                        .border_color(cx.theme().border),
                )
                .child(
                    div()
                        .h_flex()
                        .id("tabs")
                        .flex_grow()
                        .overflow_x_scroll()
                        .children(self.children),
                ),
        )
    }
}
