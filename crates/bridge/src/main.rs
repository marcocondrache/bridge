use std::sync::Arc;

use bridge::app_menus;
use gpui::{Application, AsyncApp};
use workspace::AppState;

use crate::bridge::{build_window_options, initialize_workspace};

mod bridge;

fn main() {
    let app = Application::new();

    app.run(|cx| {
        bridge::init(cx);

        let app_state = Arc::new(AppState {
            build_window_options,
        });

        AppState::set_global(Arc::downgrade(&app_state), cx);

        theme::init(cx);

        cx.set_menus(app_menus(cx));

        initialize_workspace(app_state.clone(), cx);

        cx.activate(true);

        workspace::open_new(app_state, cx)
    });
}
