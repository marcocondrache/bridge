use gpui::{
    AnyElement, Div, ElementId, InteractiveElement, ParentElement, RenderOnce, Stateful,
    StatefulInteractiveElement, div,
};
use smallvec::SmallVec;

pub struct Tab {
    base: Div,
    selected: bool,
    children: SmallVec<[AnyElement; 2]>,
}

impl Tab {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: div(),
            selected: false,
            children: SmallVec::new(),
        }
    }
}

impl InteractiveElement for Tab {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for Tab {}

impl ParentElement for Tab {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for Tab {
    fn render(self, window: &mut gpui::Window, cx: &mut gpui::App) -> impl gpui::IntoElement {
        self.base
    }
}
