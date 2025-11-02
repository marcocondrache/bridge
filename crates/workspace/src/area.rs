use gpui::{
    AppContext, Context, Entity, InteractiveElement, ParentElement, Render, Styled, WeakEntity,
    Window, div, prelude::FluentBuilder,
};
use gpui_component::{
    ActiveTheme, Sizable, StyledExt,
    tab::{Tab, TabBar},
    v_flex,
};

use crate::{Workspace, item::ItemHandle};

pub struct Area {
    workspace: WeakEntity<Workspace>,
    items: Vec<Box<dyn ItemHandle>>,
    current: usize,
}

impl Area {
    pub fn new(window: &mut Window, cx: &mut Context<Workspace>) -> Entity<Self> {
        let workspace = cx.entity().downgrade();

        cx.new(|_cx| Self {
            workspace,
            items: Vec::new(),
            current: 0,
        })
    }

    pub fn active_item_index(&self) -> usize {
        self.current
    }

    pub fn active_item(&self) -> Option<&Box<dyn ItemHandle>> {
        self.items.get(self.current)
    }

    pub fn focus_active_item(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(active_item) = self.active_item() {
            window.focus(&active_item.focus_handle(cx));
        }
    }

    pub fn activate_item(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        if index < self.items.len() {
            self.current = index;
            self.focus_active_item(window, cx);

            cx.notify();
        }
    }

    pub fn add_item(
        &mut self,
        item: Box<dyn ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.items.push(item);

        cx.notify();
    }
}

impl Render for Area {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        let theme = cx.theme();

        v_flex()
            .id("area")
            .key_context("area")
            .size_full()
            .flex_none()
            .overflow_hidden()
            .child(
                TabBar::new("Items")
                    .large()
                    .selected_index(self.active_item_index())
                    .children(self.items.iter().map(|item| Tab::new(item.tab_title(cx)))),
            )
            .child({
                div().flex().relative().overflow_hidden().map(|this| {
                    if let Some(item) = self.active_item() {
                        this.v_flex().size_full().child(item.to_any())
                    } else {
                        this.h_flex()
                            .size_full()
                            .items_center()
                            .justify_center()
                            .text_color(theme.secondary_foreground)
                            .child("Create a new HTTP request")
                    }
                })
            })
    }
}
