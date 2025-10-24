use gpui::{Axis, Entity};

use crate::pane::Pane;

#[derive(Clone)]
pub struct PaneGroup {
    pub root: Member,
}

impl PaneGroup {
    pub fn with_root(root: Member) -> Self {
        Self { root }
    }

    pub fn new(pane: Entity<Pane>) -> Self {
        Self {
            root: Member::Pane(pane),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Member {
    Axis(PaneAxis),
    Pane(Entity<Pane>),
}

#[derive(Debug, Clone)]
pub struct PaneAxis {
    pub axis: Axis,
    pub members: Vec<Member>,
}
