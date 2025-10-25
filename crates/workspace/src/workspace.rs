mod dock;
mod item;
mod pane;
mod pane_group;

use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};

use gpui::{
    App, AppContext, Context, CursorStyle, Div, Entity, EntityId, Global, Hsla, InteractiveElement,
    IntoElement, ParentElement, Render, Stateful, Styled, Task, WeakEntity, Window, WindowHandle,
    WindowOptions, div, point, prelude::FluentBuilder, px, transparent_black,
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
    panes: Vec<Entity<Pane>>,
    panes_by_item: HashMap<EntityId, WeakEntity<Pane>>,
    active_pane: Entity<Pane>,
}

impl Workspace {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let weak_self = cx.entity().downgrade();

        let center_pane = cx.new(|cx| Pane::new(weak_self.clone(), cx));

        let left_dock = Dock::new(dock::DockPlacement::Left, window, cx);
        let bottom_dock = Dock::new(dock::DockPlacement::Bottom, window, cx);

        Self {
            weak_self,
            left_dock,
            bottom_dock,
            center: PaneGroup::new(center_pane.clone()),
            panes: vec![center_pane.clone()],
            panes_by_item: Default::default(),
            active_pane: center_pane,
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
                                .bg(gpui::black())
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
                ),
            window,
            cx,
        )
    }
}

pub fn client_side_decorations(
    element: impl IntoElement,
    window: &mut Window,
    _cx: &mut App,
) -> Stateful<Div> {
    let decorations = window.window_decorations();

    div()
        .id("window-backdrop")
        .bg(transparent_black())
        .map(|div| match decorations {
            gpui::Decorations::Server => div,
            // TODO: implement
            gpui::Decorations::Client { tiling } => div
                .when(!(tiling.top || tiling.right), |div| {
                    div.rounded_tr(px(10.0))
                })
                .when(!(tiling.top || tiling.left), |div| div.rounded_tl(px(10.0)))
                .when(!(tiling.bottom || tiling.right), |div| {
                    div.rounded_br(px(10.0))
                })
                .when(!(tiling.bottom || tiling.left), |div| {
                    div.rounded_bl(px(10.0))
                })
                .when(!tiling.top, |div| div.pt(px(10.0)))
                .when(!tiling.bottom, |div| div.pb(px(10.0)))
                .when(!tiling.left, |div| div.pl(px(10.0)))
                .when(!tiling.right, |div| div.pr(px(10.0))),
        })
        .size_full()
        .child(
            div()
                .cursor(CursorStyle::Arrow)
                .map(|div| match decorations {
                    gpui::Decorations::Server => div,
                    gpui::Decorations::Client { tiling } => div
                        .when(!(tiling.top || tiling.right), |div| {
                            div.rounded_tr(px(10.0))
                        })
                        .when(!(tiling.top || tiling.left), |div| div.rounded_tl(px(10.0)))
                        .when(!(tiling.bottom || tiling.right), |div| {
                            div.rounded_br(px(10.0))
                        })
                        .when(!(tiling.bottom || tiling.left), |div| {
                            div.rounded_bl(px(10.0))
                        })
                        .when(!tiling.top, |div| div.border_t(px(1.0)))
                        .when(!tiling.bottom, |div| div.border_b(px(1.0)))
                        .when(!tiling.left, |div| div.border_l(px(1.0)))
                        .when(!tiling.right, |div| div.border_r(px(1.0)))
                        .when(!tiling.is_tiled(), |div| {
                            div.shadow(vec![gpui::BoxShadow {
                                color: Hsla {
                                    h: 0.,
                                    s: 0.,
                                    l: 0.,
                                    a: 0.4,
                                },
                                blur_radius: px(10.0) / 2.,
                                spread_radius: px(0.),
                                offset: point(px(0.0), px(0.0)),
                            }])
                        }),
                })
                .child(element),
        )
}
