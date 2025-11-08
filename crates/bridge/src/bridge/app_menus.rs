use gpui::{App, Menu, MenuItem};

pub fn app_menus(_cx: &mut App) -> Vec<Menu> {
    vec![
        Menu {
            name: "Bridge".into(),
            items: vec![],
        },
        Menu {
            name: "File".into(),
            items: vec![
                MenuItem::action("New", workspace::NewHttpEditor),
                MenuItem::action("New Window", workspace::NewWindow),
                MenuItem::separator(),
            ],
        },
    ]
}
