use std::sync::{Arc, Weak};

use gpui::{App, AppContext, Global, Render, Task, WindowHandle, WindowOptions, div};
use uuid::Uuid;

pub struct Workspace {}

impl Workspace {
    pub fn new() -> Self {
        Self {}
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

                move |_window, cx| cx.new(|_cx| Workspace::new())
            })?;

            Ok(handle)
        })
    }
}

impl Render for Workspace {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div()
    }
}

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
