pub mod area;
pub mod dock;
pub mod item;

use std::sync::{Arc, Weak};

use anyhow::Ok;
use gpui::{
    App, AppContext, Context, Div, Entity, Global, InteractiveElement, ParentElement, Render,
    Styled, Subscription, Task, WeakEntity, Window, WindowHandle, WindowOptions, div,
};
use gpui_component::{ActiveTheme, Placement, Root};
use uuid::Uuid;

use crate::{
    area::Area,
    dock::{Dock, Panel, PanelHandle},
};

pub struct AppState {
    pub build_window_options: fn(Option<Uuid>, &mut App) -> WindowOptions,
}

struct GlobalAppState(Weak<AppState>);

impl Global for GlobalAppState {}

impl AppState {
    pub fn set_global(state: Weak<AppState>, cx: &mut App) {
        cx.set_global(GlobalAppState(state));
    }
}

pub struct Workspace {
    weak_self: WeakEntity<Self>,
    right_dock: Entity<Dock>,
    bottom_dock: Entity<Dock>,
    center: Entity<Area>,
    _subscriptions: Vec<Subscription>,
}

impl Workspace {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let weak_self = cx.entity().downgrade();

        let right_dock = Dock::new(Placement::Right, cx);
        let bottom_dock = Dock::new(Placement::Bottom, cx);
        let center = Area::new(cx);

        let subscriptions = vec![];

        Self {
            weak_self,
            right_dock,
            bottom_dock,
            center,
            _subscriptions: subscriptions,
        }
    }

    pub fn add_panel<T: Panel>(
        &mut self,
        panel: Entity<T>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let placement = panel.placement(window, cx);
        let dock = self.dock_at_placement(placement);

        dock.update(cx, |dock, cx| {
            dock.add_panel(panel, cx);
        })
    }

    pub fn spawn(
        app_state: Arc<AppState>,
        cx: &mut App,
    ) -> Task<anyhow::Result<WindowHandle<Root>>> {
        let options = (app_state.build_window_options)(None, cx);

        cx.spawn(async move |cx| {
            let window = cx.open_window(options, {
                let _app_state = app_state.clone();

                move |window, cx| {
                    let workspace = cx.new(|cx| Workspace::new(window, cx));

                    cx.new(|cx| Root::new(workspace.into(), window, cx))
                }
            })?;

            window.update(cx, |_workspace, window, _cx| {
                window.activate_window();
            })?;

            Ok(window)
        })
    }

    fn dock_at_placement(&self, placement: Placement) -> &Entity<Dock> {
        match placement {
            Placement::Right => &self.right_dock,
            Placement::Bottom => &self.bottom_dock,
            _ => &self.right_dock,
        }
    }

    fn render_dock(&self, dock: &Entity<Dock>) -> Option<Div> {
        Some(
            div()
                .flex()
                .flex_none()
                .overflow_hidden()
                .child(dock.clone()),
        )
    }
}

pub fn open_new(app_state: Arc<AppState>, cx: &mut App) {
    let task = Workspace::spawn(app_state, cx);

    cx.spawn(async move |_| {
        let _ = task.await;
    })
    .detach();
}

impl Render for Workspace {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let theme = cx.theme().clone();

        // TODO: Extract into separate layers
        div()
            .id("root")
            .key_context("root")
            .relative()
            .size_full()
            .flex()
            .flex_col()
            .justify_start()
            .items_start()
            .text_color(theme.foreground)
            .overflow_hidden()
            .child(
                div()
                    .id("workspace")
                    .bg(theme.background)
                    .relative()
                    .flex_1()
                    .w_full()
                    .flex()
                    .flex_col()
                    .overflow_hidden()
                    .border_t_1()
                    .border_b_1()
                    .border_color(theme.border)
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .h_full()
                            .children(self.render_dock(&self.right_dock))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .flex_1()
                                    .overflow_hidden()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .flex_1()
                                            .child(self.center.clone()),
                                    )
                                    .children(self.render_dock(&self.bottom_dock)),
                            ),
                    ),
            )
    }
}
