mod app_menus;

use std::sync::Arc;

use collection::Collection;
use gpui::{
    App, AppContext, Context, TitlebarOptions, Window, WindowKind, WindowOptions, actions, point,
    px,
};
use uuid::Uuid;

pub use app_menus::*;
use workspace::{AppState, Workspace};

pub fn init(cx: &mut App) {}

pub fn initialize_workspace(state: Arc<AppState>, cx: &mut App) {
    cx.observe_new(move |workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };

        let handle = cx.entity();

        initialize_panels(window, cx);
    })
    .detach();
}

pub fn initialize_panels(window: &mut Window, cx: &mut Context<Workspace>) {
    cx.spawn_in(window, async move |handle, cx| {
        let collection_panel = cx.new(|_| Collection {}).unwrap();

        handle.update_in(cx, |workspace, window, cx| {
            workspace.add_panel(collection_panel, window, cx);
        })
    })
    .detach();
}

pub fn build_window_options(display_uuid: Option<Uuid>, cx: &mut App) -> WindowOptions {
    let display = display_uuid.and_then(|uuid| {
        cx.displays()
            .into_iter()
            .find(|display| display.uuid().ok() == Some(uuid))
    });

    let use_system_window_tabs = true;

    WindowOptions {
        titlebar: Some(TitlebarOptions {
            title: None,
            appears_transparent: true,
            traffic_light_position: Some(point(px(9.0), px(9.0))),
        }),
        window_bounds: None,
        focus: false,
        show: false,
        kind: WindowKind::Normal,
        is_movable: true,
        display_id: display.map(|display| display.id()),
        // window_background: cx.theme().window_background_appearance(),
        // app_id: Some(app_id.to_owned()),
        window_decorations: Some(gpui::WindowDecorations::Server),
        window_min_size: Some(gpui::Size {
            width: px(360.0),
            height: px(240.0),
        }),
        tabbing_identifier: if use_system_window_tabs {
            Some(String::from("bridge"))
        } else {
            None
        },
        ..Default::default()
    }
}
