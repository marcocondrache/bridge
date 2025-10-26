use gpui::{InteractiveElement, IntoElement, ParentElement, Styled, div};
use theme::ActiveTheme;

pub fn root(e: impl IntoElement, cx: &mut gpui::App) -> impl IntoElement {
    let theme = cx.theme();
    let colors = theme.colors();

    div()
        .id("root")
        .key_context("root")
        .relative()
        .size_full()
        .flex()
        .flex_col()
        .justify_start()
        .items_start()
        .text_color(colors.foreground)
        .overflow_hidden()
        .child(e)
}
