use std::sync::Arc;

use gpui::{AppContext, Context, Entity, Focusable, Render, Window, div};

use crate::Workspace;

pub trait Panel: Focusable + Render + Sized {}

pub trait PanelHandle: Send + Sync {}

impl<T> PanelHandle for Entity<T> where T: Panel {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DockPosition {
    Left,
    Bottom,
    Right,
}

pub struct Dock {
    position: DockPosition,
    panel_entries: Vec<Arc<dyn PanelHandle>>,
    is_open: bool,
    active_panel_index: Option<usize>,
}

impl Dock {
    pub fn new(
        position: DockPosition,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        cx.new(|cx| Self {
            position,
            panel_entries: Vec::new(),
            is_open: false,
            active_panel_index: None,
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
