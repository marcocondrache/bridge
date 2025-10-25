use gpui::{App, Menu};

pub fn app_menus(_cx: &mut App) -> Vec<Menu> {
    vec![Menu {
        name: "Bridge".into(),
        items: vec![],
    }]
}
