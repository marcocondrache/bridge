use gpui::{AppContext, Context, Entity, ParentElement, Render, Styled, Window};
use gpui_component::{
    input::{Input, InputState},
    v_flex,
};

pub struct Body {
    state: Entity<InputState>,
}

impl Body {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let state = cx.new(|cx| InputState::new(window, cx).code_editor("json"));

        Self { state }
    }
}

impl Render for Body {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        v_flex().h_full().child(Input::new(&self.state))
    }
}
