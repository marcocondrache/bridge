use gpui::{
    AnyView, App, AppContext, Context, Entity, EntityId, FocusHandle, Focusable,
    InteractiveElement, ParentElement, Render, SharedString, Styled, WeakEntity, Window, div,
    prelude::FluentBuilder,
};
use gpui_component::{
    ActiveTheme, Sizable, StyledExt,
    button::Button,
    h_flex,
    tab::{Tab, TabBar},
    v_flex,
};

use crate::{NewHttpEditor, Workspace};

pub trait Item: Focusable + Render + Sized {
    fn tab_title(&self, cx: &App) -> SharedString;
}

pub trait ItemHandle {
    fn to_any(&self) -> AnyView;
    fn tab_title(&self, cx: &App) -> SharedString;
    fn item_id(&self) -> EntityId;
    fn focus_handle(&self, cx: &App) -> FocusHandle;
}

impl<T: Item> ItemHandle for Entity<T> {
    fn to_any(&self) -> AnyView {
        self.clone().into()
    }

    fn tab_title(&self, cx: &App) -> SharedString {
        self.read(cx).tab_title(cx)
    }

    fn item_id(&self) -> EntityId {
        self.entity_id()
    }

    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.read(cx).focus_handle(cx)
    }
}

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
                            .child(Button::new("test").on_click(|_, window, cx| {
                                window.dispatch_action(Box::new(NewHttpEditor), cx);
                            }))
                    }
                })
            })
    }
}
