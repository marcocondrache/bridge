use std::{cmp::Ordering, convert::identity, sync::Arc};

use gpui::{
    AnyView, App, AppContext, Axis, Context, Entity, ParentElement, Pixels, Render, SharedString,
    StyleRefinement, Styled, Subscription, WeakEntity, Window, div, prelude::FluentBuilder, px,
};
use gpui_component::{ActiveTheme, Placement, StyledExt};

use crate::Workspace;

pub trait Panel: Render + Sized {
    fn priority(&self) -> u32;

    fn tab_name(&self, cx: &App) -> Option<SharedString> {
        None
    }

    fn closable(&self, cx: &App) -> bool {
        true
    }

    fn visible(&self, cx: &App) -> bool {
        true
    }
}

pub trait PanelHandle: Send + Sync {
    fn priority(&self, cx: &App) -> u32;
    fn tab_name(&self, cx: &App) -> Option<SharedString>;
    fn closable(&self, cx: &App) -> bool;
    fn visible(&self, cx: &App) -> bool;
    fn to_any(&self) -> AnyView;
}

impl<T: Panel> PanelHandle for Entity<T> {
    fn priority(&self, cx: &App) -> u32 {
        self.read(cx).priority()
    }

    fn to_any(&self) -> AnyView {
        self.clone().into()
    }

    fn tab_name(&self, cx: &App) -> Option<SharedString> {
        self.read(cx).tab_name(cx)
    }

    fn closable(&self, cx: &App) -> bool {
        self.read(cx).closable(cx)
    }

    fn visible(&self, cx: &App) -> bool {
        self.read(cx).visible(cx)
    }
}

pub struct Dock {
    is_open: bool,
    placement: Placement,
    workspace: WeakEntity<Workspace>,
    items: Vec<(Arc<dyn PanelHandle>, Subscription)>,
    size: Pixels,
    current_index: Option<usize>,
}

impl Dock {
    pub fn new(placement: Placement, cx: &mut Context<Workspace>) -> Entity<Self> {
        let workspace = cx.entity();

        cx.new(|_cx| Self {
            placement,
            workspace: workspace.downgrade(),
            is_open: false,
            items: Vec::new(),
            size: px(200.),
            current_index: None,
        })
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn toggle_open(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.set_open(!self.is_open, window, cx);
    }

    pub fn set_open(&mut self, open: bool, window: &mut Window, cx: &mut Context<Self>) {
        self.is_open = open;
        cx.notify();
    }

    pub fn add_panel<T: Panel>(&mut self, panel: Entity<T>, cx: &mut Context<Self>) -> usize {
        let subscription = cx.observe(&panel, |_, _, cx| cx.notify());

        let index = self
            .items
            .binary_search_by_key(&panel.priority(cx), |item| item.0.priority(cx))
            .unwrap_or_else(identity);

        if let Some(current) = self.current_index.as_mut()
            && *current >= index
        {
            *current += 1;
        }

        self.items.insert(index, (Arc::new(panel), subscription));

        cx.notify();

        index
    }

    pub fn remove_panel(&mut self, index: usize) {
        let _ = self.items.remove(index);

        if let Some(current) = self.current_index.as_mut() {
            match index.cmp(current) {
                Ordering::Less => *current -= 1,
                Ordering::Equal => self.current_index = None,
                _ => {}
            }
        }
    }

    pub fn display_panel(&mut self, index: usize, cx: &mut Context<Self>) {
        self.current_index = Some(index);

        cx.notify();
    }

    pub fn visibile_panel(&self) -> Option<&Arc<dyn PanelHandle>> {
        self.is_open
            .then(|| self.current_index.and_then(|index| self.items.get(index)))
            .flatten()
            .map(|e| &e.0)
    }

    pub fn active_panel(&self) -> Option<&Arc<dyn PanelHandle>> {
        self.current_index
            .and_then(|index| self.items.get(index))
            .map(|e| &e.0)
    }
}

impl Render for Dock {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        if let Some(panel) = self.visibile_panel() {
            div()
                .flex()
                .bg(cx.theme().background)
                .border_color(cx.theme().border)
                .overflow_hidden()
                .map(|this| match self.placement.axis() {
                    Axis::Vertical => this.h_full().flex_row().w(self.size),
                    Axis::Horizontal => this.w_full().flex_col().h(self.size),
                })
                .child(
                    panel
                        .to_any()
                        .cached(StyleRefinement::default().v_flex().size_full()),
                )
        } else {
            div()
        }
    }
}
