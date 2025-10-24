mod dock;
mod item;
mod pane;
mod pane_group;

use std::sync::{Arc, Weak};

use gpui::{
    App, AppContext, Context, CursorStyle, Div, Entity, Global, InteractiveElement, IntoElement,
    ParentElement, Render, Styled, Task, WeakEntity, Window, WindowHandle, WindowOptions, div,
    prelude::FluentBuilder,
};
use uuid::Uuid;

use crate::{dock::Dock, pane::Pane, pane_group::PaneGroup};

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
}

impl Workspace {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let weak_self = cx.entity().downgrade();

        let center_pane = cx.new(|cx| Pane::new());

        let left_dock = Dock::new(dock::DockPosition::Left, window, cx);
        let bottom_dock = Dock::new(dock::DockPosition::Bottom, window, cx);

        Self {
            weak_self,
            left_dock,
            bottom_dock,
            center: PaneGroup::new(center_pane),
        }
    }

    pub fn new_local(
        app_state: Arc<AppState>,
        _requesting_window: Option<WindowHandle<Workspace>>,
        cx: &mut App,
    ) -> Task<anyhow::Result<(WindowHandle<Workspace>)>> {
        let options = (app_state.build_window_options)(None, cx);

        cx.spawn(async move |cx| {
            let handle = cx.open_window(options, {
                let _app_state = app_state.clone();

                move |window, cx| cx.new(|cx| Workspace::new(window, cx))
            })?;

            Ok(handle)
        })
    }

    fn render_dock(&self, dock: &Entity<Dock>, window: &mut Window, cx: &mut App) -> Option<Div> {
        Some(div())
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
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        // TODO: Extract into separate layers
        client_side_decorations(
            div()
                .id("root")
                .relative()
                .size_full()
                .flex()
                .flex_col()
                .gap_0()
                .justify_start()
                .items_start()
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
                                .relative()
                                .flex_1()
                                .w_full()
                                .flex()
                                .flex_col()
                                .overflow_hidden()
                                .border_t_1()
                                .border_b_1()
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .h_full()
                                        .children(self.render_dock(&self.left_dock, window, cx))
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
                                                .children(self.render_dock(
                                                    &self.bottom_dock,
                                                    window,
                                                    cx,
                                                )),
                                        ),
                                ),
                        ),
                ),
            window,
            cx,
        )
    }
}

pub fn client_side_decorations(
    element: impl IntoElement,
    window: &mut Window,
    cx: &mut App,
) -> Div {
    let decorations = window.window_decorations();

    div()
        .map(|div| match decorations {
            gpui::Decorations::Server => div,
            gpui::Decorations::Client { tiling } => div,
        })
        .size_full()
        .child(div().cursor(CursorStyle::Arrow).child(element))
}
