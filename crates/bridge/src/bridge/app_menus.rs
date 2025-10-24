use gpui::{App, Menu};

pub fn app_menus(cx: &mut App) -> Vec<Menu> {
    vec![Menu {
        name: "Bridge".into(),
        items: vec![],
    }]
}
