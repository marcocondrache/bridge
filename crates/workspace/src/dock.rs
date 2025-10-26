use std::{cmp::Ordering, convert::identity, sync::Arc};

use gpui::{
    AnyView, App, AppContext, Axis, Context, Entity, Focusable, ParentElement, Render,
    StyleRefinement, Styled, Subscription, WeakEntity, Window, div, prelude::FluentBuilder,
};
use theme::ActiveTheme;
use ui::{placement::Placement, traits::styled_ext::StyledExt};

use crate::Workspace;

pub trait Panel: Focusable + Render + Sized {
    fn priority(&self) -> u32;
}

pub trait PanelHandle: Send + Sync {
    fn priority(&self, cx: &App) -> u32;
    fn to_any(&self) -> AnyView;
}

impl<T: Panel> PanelHandle for Entity<T> {
    fn priority(&self, cx: &App) -> u32 {
        self.read(cx).priority()
    }

    fn to_any(&self) -> AnyView {
        self.clone().into()
    }
}

pub struct Dock {
    placement: Placement,
    workspace: WeakEntity<Workspace>,
    is_open: bool,
    items: Vec<(Arc<dyn PanelHandle>, Subscription)>,
    current: Option<usize>,
}

impl Dock {
    pub fn new(placement: Placement, cx: &mut Context<Workspace>) -> Entity<Self> {
        let workspace = cx.entity();

        cx.new(|_cx| Self {
            placement,
            workspace: workspace.downgrade(),
            is_open: false,
            items: Vec::new(),
            current: None,
        })
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn add_panel<T: Panel>(&mut self, panel: Entity<T>, cx: &mut Context<Workspace>) -> usize {
        let subscription = cx.observe(&panel, |_, _, cx| cx.notify());

        let index = self
            .items
            .binary_search_by_key(&panel.priority(cx), |item| item.0.priority(cx))
            .unwrap_or_else(identity);

        if let Some(current) = self.current.as_mut()
            && *current >= index
        {
            *current += 1;
        }

        self.items.insert(index, (Arc::new(panel), subscription));

        cx.notify();

        index
    }

    pub fn remove_panel(&mut self, index: usize) {
        self.items.remove(index);

        if let Some(current) = self.current.as_mut() {
            match index.cmp(current) {
                Ordering::Less => *current -= 1,
                Ordering::Equal => self.current = None,
                _ => {}
            }
        }
    }

    pub fn display_panel(&mut self, index: usize) {
        self.current = Some(index);
    }

    pub fn visibile_panel(&self) -> Option<&Arc<dyn PanelHandle>> {
        self.is_open
            .then(|| self.current.and_then(|index| self.items.get(index)))
            .flatten()
            .map(|e| &e.0)
    }

    pub fn active_panel(&self) -> Option<&Arc<dyn PanelHandle>> {
        self.current
            .and_then(|index| self.items.get(index))
            .map(|e| &e.0)
    }
}

impl Render for Dock {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        if let Some(panel) = self.visibile_panel() {
            div()
                .flex()
                .bg(cx.theme().colors().background)
                .border_color(cx.theme().colors().border)
                .overflow_hidden()
                .map(|this| match self.placement.axis() {
                    Axis::Vertical => this.h_full().flex_row(),
                    Axis::Horizontal => this.w_full().flex_col(),
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
