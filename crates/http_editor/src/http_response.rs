use gpui::{AppContext, Context, Entity, ParentElement, Render, Styled, Window};
use gpui_component::{
    input::{InputState, TextInput},
    v_flex,
};

// TODO: implement into the bottom dock
pub struct HttpResponse {
    state: Entity<InputState>,
}

impl HttpResponse {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let state = cx.new(|cx| {
            InputState::new(window, cx)
                .code_editor("json")
                .line_number(true)
        });

        Self { state }
    }
}

impl Render for HttpResponse {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        v_flex()
            .h_full()
            .child(TextInput::new(&self.state).disabled(true))
    }
}
