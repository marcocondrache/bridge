use gpui::{
    AppContext, Context, Entity, ParentElement, Render, SharedString, Styled, Window,
    prelude::FluentBuilder,
};
use gpui_component::{
    input::{Input, InputState},
    select::{Select, SelectItem},
    v_flex,
};

use crate::body_type_selector::{BodyTypeSelector, body_type_selector};

#[derive(Debug, Default, Clone, PartialEq)]
pub enum BodyType {
    #[default]
    None,
    FormData,
    FormUrlEncoded,
    Raw,
}

impl BodyType {
    pub fn all() -> [Self; 4] {
        [Self::None, Self::FormData, Self::FormUrlEncoded, Self::Raw]
    }
}

impl From<BodyType> for SharedString {
    fn from(kind: BodyType) -> Self {
        match kind {
            BodyType::None => "None".into(),
            BodyType::FormData => "FormData".into(),
            BodyType::FormUrlEncoded => "FormUrlEncoded".into(),
            BodyType::Raw => "Raw".into(),
        }
    }
}

impl SelectItem for BodyType {
    type Value = Self;

    fn title(&self) -> gpui::SharedString {
        self.clone().into()
    }

    fn value(&self) -> &Self::Value {
        self
    }
}

enum ActiveView {
    None,
    Raw { editor: Entity<InputState> },
}

impl ActiveView {
    pub fn raw(window: &mut Window, cx: &mut Context<BodyTab>) -> Self {
        let editor = cx.new(|cx| InputState::new(window, cx).code_editor(""));

        Self::Raw { editor }
    }
}

pub struct BodyTab {
    selector: Entity<BodyTypeSelector>,
    active_view: ActiveView,
}

impl BodyTab {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let selector = cx.new(|cx| body_type_selector(window, cx));

        Self {
            selector,
            active_view: ActiveView::None,
        }
    }
}

impl BodyTab {
    fn render_selector(
        &self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        Select::new(&self.selector)
    }
}

impl Render for BodyTab {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        v_flex()
            .h_full()
            .gap_4()
            .child(self.render_selector(window, cx))
            .map(|parent| match &self.active_view {
                ActiveView::None => parent,
                ActiveView::Raw { editor } => parent.child(Input::new(&editor)),
            })
    }
}
