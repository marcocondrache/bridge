use std::sync::Arc;

use gpui::{Application, AsyncApp};
use workspace::AppState;

use crate::bridge::build_window_options;

mod bridge;

fn main() {
    let app = Application::new();

    app.run(|cx| {
        bridge::init(cx);

        let app_state = Arc::new(AppState {
            build_window_options,
        });

        AppState::set_global(Arc::downgrade(&app_state), cx);

        cx.activate(true);

        cx.spawn({
            let app_state = app_state.clone();
            async move |cx| {
                restore_or_create_workspace(app_state, cx).await;
            }
        })
        .detach();
    });
}

async fn restore_or_create_workspace(app_state: Arc<AppState>, cx: &mut AsyncApp) {
    let _ = cx.update(|cx| workspace::open_new(app_state, cx));
}
