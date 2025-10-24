use std::sync::{Arc, Mutex};

use gpui::{AnyElement, Axis, Element, Entity, div};

use crate::pane::Pane;

#[derive(Clone, Copy, Debug)]
pub enum SplitDirection {
    Up,
    Down,
    Left,
    Right,
}

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

    pub fn render(&self) -> AnyElement {
        self.root.render()
    }
}

#[derive(Debug, Clone)]
pub enum Member {
    Axis(PaneAxis),
    Pane(Entity<Pane>),
}

impl Member {
    pub fn new_axis(
        old_pane: Entity<Pane>,
        new_pane: Entity<Pane>,
        direction: SplitDirection,
    ) -> Self {
        use Axis::*;
        use SplitDirection::*;

        let axis = match direction {
            Up | Down => Vertical,
            Left | Right => Horizontal,
        };

        let members = match direction {
            Up | Left => vec![Member::Pane(new_pane), Member::Pane(old_pane)],
            Down | Right => vec![Member::Pane(old_pane), Member::Pane(new_pane)],
        };

        Member::Axis(PaneAxis::new(axis, members))
    }

    pub fn render(&self) -> AnyElement {
        match self {
            Member::Axis(_axis) => div(),
            Member::Pane(_pane) => div(),
        }
        .into_any()
    }
}

#[derive(Debug, Clone)]
pub struct PaneAxis {
    pub axis: Axis,
    pub members: Vec<Member>,
    pub ratios: Arc<Mutex<Vec<f32>>>,
}

impl PaneAxis {
    pub fn new(axis: Axis, members: Vec<Member>) -> Self {
        let ratios = Arc::new(Mutex::new(vec![1.; members.len()]));

        Self {
            axis,
            members,
            ratios,
        }
    }
}
