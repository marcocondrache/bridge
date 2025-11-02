use gpui::{AppContext, Context, Entity, IntoElement, Render, Window};
use gpui_component::{
    Sizable,
    input::{InputState, TextInput},
};

pub struct HttpTarget {
    state: Entity<InputState>,
}

impl HttpTarget {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let state = cx.new(|cx| InputState::new(window, cx).placeholder("Enter URL"));

        Self { state }
    }
}

impl Render for HttpTarget {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        TextInput::new(&self.state).large()
    }
}
