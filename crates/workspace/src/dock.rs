use gpui::{AppContext, Context, Entity, Render, Window, div};

use crate::Workspace;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DockPosition {
    Left,
    Bottom,
    Right,
}

pub struct Dock {
    position: DockPosition,
    is_open: bool,
}

impl Dock {
    pub fn new(
        position: DockPosition,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        cx.new(|cx| Self {
            position,
            is_open: false,
        })
    }

    pub fn position(&self) -> DockPosition {
        self.position
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }
}

impl Render for Dock {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        div()
    }
}
