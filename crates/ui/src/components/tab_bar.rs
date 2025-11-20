use gpui::prelude::FluentBuilder;
use gpui::{AnyElement, App, ElementId, ScrollHandle, Window, div};
use gpui_component::ActiveTheme;
use smallvec::SmallVec;
use ui_component::{Component, titled_group, variant};
use ui_macros::RegisterComponent;

use crate::components::tab::Tab;
use crate::prelude::*;

#[derive(IntoElement, RegisterComponent)]
pub struct TabBar {
    id: ElementId,
    children: SmallVec<[AnyElement; 2]>,
    scroll_handle: Option<ScrollHandle>,
}

impl TabBar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            children: SmallVec::new(),
            scroll_handle: None,
        }
    }

    pub fn track_scroll(mut self, scroll_handle: ScrollHandle) -> Self {
        self.scroll_handle = Some(scroll_handle);
        self
    }
}

impl ParentElement for TabBar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for TabBar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        div()
            .id(self.id)
            .group("tab_bar")
            .flex()
            .flex_none()
            .w_full()
            .bg(theme.tab_bar)
            .child(
                div()
                    .relative()
                    .flex_1()
                    .h_full()
                    .overflow_x_hidden()
                    .child(
                        div()
                            .absolute()
                            .top_0()
                            .left_0()
                            .size_full()
                            .border_b_1()
                            .border_color(theme.border),
                    )
                    .child(
                        div()
                            .h_flex()
                            .id("tabs")
                            .flex_grow()
                            .overflow_x_scroll()
                            .when_some(self.scroll_handle, |cx, scroll_handle| {
                                cx.track_scroll(&scroll_handle)
                            })
                            .children(self.children),
                    ),
            )
    }
}

impl Component for TabBar {
    fn showcase(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        Some(
            div()
                .v_flex()
                .gap_6()
                .children(vec![titled_group(
                    "Basic Usage",
                    vec![
                        variant(
                            "Empty TabBar",
                            TabBar::new("empty_tab_bar").into_any_element(),
                        ),
                        variant(
                            "With Tabs",
                            TabBar::new("tab_bar_with_tabs")
                                .child(Tab::new("tab1").child("Tab 1"))
                                .child(Tab::new("tab2").child("Tab 2"))
                                .child(Tab::new("tab3").child("Tab 3"))
                                .into_any_element(),
                        ),
                    ],
                )])
                .into_any_element(),
        )
    }
}
