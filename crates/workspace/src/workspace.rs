use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};

use gpui::{
    App, AppContext, Context, Div, Entity, EntityId, Global, InteractiveElement, IntoElement,
    ParentElement, Render, Styled, Subscription, Task, WeakEntity, Window, WindowHandle,
    WindowOptions, div,
};
use theme::{ActiveTheme, GlobalTheme, SystemAppearance};
use uuid::Uuid;

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
    center: PaneGroup,
    left_dock: Entity<Dock>,
    bottom_dock: Entity<Dock>,
    panes: Vec<Entity<Pane>>,
    panes_by_item: HashMap<EntityId, WeakEntity<Pane>>,
    active_pane: Entity<Pane>,
    _subscriptions: Vec<Subscription>,
}

impl Workspace {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let weak_self = cx.entity().downgrade();

        let center_pane = cx.new(|cx| Pane::new(weak_self.clone(), cx));

        let left_dock = Dock::new(dock::DockPlacement::Left, window, cx);
        let bottom_dock = Dock::new(dock::DockPlacement::Bottom, window, cx);

        let subscriptions = vec![cx.observe_window_appearance(window, |_, window, cx| {
            let window_appearance = window.appearance();

            *SystemAppearance::global_mut(cx) = SystemAppearance(window_appearance.into());

            GlobalTheme::reload_theme(cx);
        })];

        Self {
            weak_self,
            left_dock,
            bottom_dock,
            center: PaneGroup::new(center_pane.clone()),
            panes: vec![center_pane.clone()],
            panes_by_item: Default::default(),
            active_pane: center_pane,
            _subscriptions: subscriptions,
        }
    }

    pub fn new_local(
        app_state: Arc<AppState>,
        _requesting_window: Option<WindowHandle<Workspace>>,
        cx: &mut App,
    ) -> Task<anyhow::Result<(WindowHandle<Workspace>)>> {
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

pub fn open_new(
    // open_options: OpenOptions,
    app_state: Arc<AppState>,
    cx: &mut App,
) -> Task<anyhow::Result<()>> {
    let task = Workspace::new_local(app_state, None, cx);

    cx.spawn(async move |_cx| {
        let _workspace = task.await?;

        Ok(())
    })
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
        div()
            .id("root")
            .relative()
            .size_full()
            .flex()
            .flex_col()
            .gap_0()
            .justify_start()
            .items_start()
            .text_color(colors.foreground)
            .overflow_hidden()
            .child(
                div()
                    .id("ephemeral_overlay")
                    .size_full()
                    .relative()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .child(
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
                                                    .child(self.center.render()),
                                            )
                                            .children(self.render_dock(&self.bottom_dock)),
                                    ),
                            ),
                    ),
            )
    }
}
