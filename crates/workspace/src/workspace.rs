mod area;
mod dock;
mod item;

use std::sync::{Arc, Weak};

use anyhow::Ok;
use gpui::{
    App, AppContext, Context, Div, Entity, Global, InteractiveElement, ParentElement, Render,
    Styled, Subscription, Task, WeakEntity, Window, WindowHandle, WindowOptions, div,
};
use theme::{ActiveTheme, GlobalTheme, SystemAppearance};

use ui::{components::root::root, placement::Placement};
use uuid::Uuid;

use crate::{area::Area, dock::Dock};

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
    left_dock: Entity<Dock>,
    bottom_dock: Entity<Dock>,
    center: Entity<Area>,
    _subscriptions: Vec<Subscription>,
}

impl Workspace {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let weak_self = cx.entity().downgrade();

        let left_dock = Dock::new(Placement::Left, cx);
        let bottom_dock = Dock::new(Placement::Bottom, cx);
        let center = Area::new(cx);

        let subscriptions = vec![cx.observe_window_appearance(window, |_, window, cx| {
            let window_appearance = window.appearance();

            *SystemAppearance::global_mut(cx) = SystemAppearance(window_appearance.into());

            GlobalTheme::reload_theme(cx);
        })];

        Self {
            weak_self,
            left_dock,
            bottom_dock,
            center,
            _subscriptions: subscriptions,
        }
    }

    pub fn spawn(
        app_state: Arc<AppState>,
        _requesting_window: Option<WindowHandle<Workspace>>,
        cx: &mut App,
    ) -> Task<anyhow::Result<WindowHandle<Workspace>>> {
        let options = (app_state.build_window_options)(None, cx);

        cx.spawn(async move |cx| {
            let window = cx.open_window(options, {
                let _app_state = app_state.clone();

                move |window, cx| cx.new(|cx| Workspace::new(window, cx))
            })?;

            window.update(cx, |_workspace, window, _cx| {
                window.activate_window();
            })?;

            Ok(window)
        })
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
    let task = Workspace::spawn(app_state, None, cx);

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
        let colors = theme.colors();

        // TODO: Extract into separate layers
        root(
            div()
                .id("workspace")
                .bg(colors.background)
                .relative()
                .flex_1()
                .w_full()
                .flex()
                .flex_col()
                .overflow_hidden()
                .border_t_1()
                .border_b_1()
                .border_color(colors.border)
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .h_full()
                        .children(self.render_dock(&self.left_dock))
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
            cx,
        )
    }
}
