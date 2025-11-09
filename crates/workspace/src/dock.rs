use std::{cmp::Ordering, convert::identity, sync::Arc};

use gpui::{
    AnyView, App, AppContext, Axis, Context, Entity, FocusHandle, Focusable, InteractiveElement,
    ParentElement, Pixels, Render, SharedString, StyleRefinement, Styled, Subscription, WeakEntity,
    Window, div, prelude::FluentBuilder, px,
};
use gpui_component::{ActiveTheme, StyledExt};
use ui::{components::resize_handle::ResizeHandle, utils::placement::Placement};

use crate::{DraggedDock, Workspace};

pub trait Panel: Render + Sized {
    fn priority(&self) -> u32;
    fn placement(&self, window: &Window, cx: &App) -> Placement;

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
    fn placement(&self, window: &Window, cx: &App) -> Placement;
    fn tab_name(&self, cx: &App) -> Option<SharedString>;
    fn closable(&self, cx: &App) -> bool;
    fn visible(&self, cx: &App) -> bool;
    fn to_any(&self) -> AnyView;
}

impl<T: Panel> PanelHandle for Entity<T> {
    fn priority(&self, cx: &App) -> u32 {
        self.read(cx).priority()
    }

    fn placement(&self, window: &Window, cx: &App) -> Placement {
        self.read(cx).placement(window, cx)
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
    size: Option<Pixels>,
    placement: Placement,
    workspace: WeakEntity<Workspace>,
    items: Vec<(Arc<dyn PanelHandle>, Subscription)>,
    active_panel_index: Option<usize>,
    focus_handle: FocusHandle,
}

impl Focusable for Dock {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Dock {
    pub fn new(placement: Placement, cx: &mut Context<Workspace>) -> Entity<Self> {
        let focus_handle = cx.focus_handle();
        let workspace = cx.entity();

        cx.new(|_cx| Self {
            placement,
            workspace: workspace.downgrade(),
            is_open: true,
            size: None,
            items: Vec::new(),
            active_panel_index: None,
            focus_handle,
        })
    }

    pub fn placement(&self) -> Placement {
        self.placement
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

        if let Some(current) = self.active_panel_index.as_mut()
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

        if let Some(current) = self.active_panel_index.as_mut() {
            match index.cmp(current) {
                Ordering::Less => *current -= 1,
                Ordering::Equal => self.active_panel_index = None,
                _ => {}
            }
        }
    }

    pub fn display_panel(&mut self, index: usize, cx: &mut Context<Self>) {
        self.active_panel_index = Some(index);

        cx.notify();
    }

    pub fn visibile_panel(&self) -> Option<&Arc<dyn PanelHandle>> {
        self.is_open
            .then(|| {
                self.active_panel_index
                    .and_then(|index| self.items.get(index))
            })
            .flatten()
            .map(|e| &e.0)
    }

    pub fn active_panel(&self) -> Option<&Arc<dyn PanelHandle>> {
        self.active_panel_index
            .and_then(|index| self.items.get(index))
            .map(|e| &e.0)
    }

    pub fn resize(&mut self, size: Option<Pixels>, cx: &mut Context<Self>) {
        self.size = size;
        cx.notify();
    }
}

impl Render for Dock {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        if let Some(panel) = self.visibile_panel() {
            let size = self.size.unwrap_or(px(200.0));
            let dock = cx.entity().downgrade();
            let resize_handle = ResizeHandle::new(DraggedDock(self.placement()), self.placement())
                .on_double_click(move |_, cx| {
                    dock.update(cx, |dock, cx| {
                        dock.resize(None, cx);
                    })
                    .ok();
                });

            div()
                .track_focus(&self.focus_handle(cx))
                .flex()
                .bg(cx.theme().background)
                .border_color(cx.theme().border)
                .overflow_hidden()
                .map(|this| match self.placement.axis() {
                    Axis::Vertical => this.h(size).w_full().flex_col(),
                    Axis::Horizontal => this.w(size).h_full().flex_row(),
                })
                .map(|this| match self.placement() {
                    Placement::Left => this.border_r_1(),
                    Placement::Right => this.border_l_1(),
                    Placement::Bottom => this.border_t_1(),
                    Placement::Top => this.border_b_1(),
                })
                .child(
                    div()
                        .map(|this| match self.placement().axis() {
                            Axis::Horizontal => this.min_w(size).h_full(),
                            Axis::Vertical => this.min_h(size).w_full(),
                        })
                        .child(
                            panel
                                .to_any()
                                .cached(StyleRefinement::default().v_flex().size_full()),
                        ),
                )
                .child(resize_handle)
        } else {
            div()
        }
    }
}
