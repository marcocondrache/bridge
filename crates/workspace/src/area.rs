use gpui::{
    AppContext, Context, Entity, InteractiveElement, ParentElement, Render, Styled, WeakEntity,
    div, prelude::FluentBuilder,
};
use gpui_component::{StyledExt, v_flex};
use http_ui::{http_editor::HttpEditor, http_method_selector::HttpMethodSelector};

use crate::{Workspace, item::ItemHandle};

pub struct Area {
    workspace: WeakEntity<Workspace>,
    items: Vec<Box<dyn ItemHandle>>,
    current: usize,
}

impl Area {
    pub fn new(cx: &mut Context<Workspace>) -> Entity<Self> {
        let workspace = cx.entity().downgrade();

        cx.new(|_cx| Self {
            workspace,
            items: Vec::new(),
            current: 0,
        })
    }

    pub fn active_item(&self) -> Option<&Box<dyn ItemHandle>> {
        self.items.get(self.current)
    }
}

impl Render for Area {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        let editor = cx.new(|cx| HttpEditor::new(window, cx));

        v_flex()
            .id("area")
            .key_context("area")
            .size_full()
            .flex_none()
            .overflow_hidden()
            .child({
                div().flex().relative().overflow_hidden().map(|this| {
                    if let Some(item) = self.active_item() {
                        this.v_flex().size_full().child(item.to_any())
                    } else {
                        this.h_flex().size_full().justify_center().child(editor)
                    }
                })
            })
    }
}
