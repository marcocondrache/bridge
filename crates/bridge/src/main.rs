use std::sync::Arc;

use assets::Assets;
use bridge::app_menus;
use gpui::{Application, KeyBinding};
use workspace::{AppState, NewHttpEditor};

use crate::bridge::{build_window_options, initialize_workspace};

mod bridge;

fn main() {
    let app = Application::new().with_assets(Assets);

    app.run(|cx| {
        gpui_component::init(cx);

        ui::components::input::init(cx);
        ui_component::init();

        http_client::init(cx);
        settings::init(cx);

        bridge::init(cx);

        title_bar::init(cx);
        http_editor::init(cx);

        let app_state = Arc::new(AppState {
            build_window_options,
        });

        AppState::set_global(Arc::downgrade(&app_state), cx);

        cx.bind_keys([KeyBinding::new("enter", NewHttpEditor, None)]);

        let menus = app_menus(cx);
        cx.set_menus(menus);

        initialize_workspace(app_state.clone(), cx);

        cx.activate(true);

        workspace::open_new(app_state, cx);
    });
}
