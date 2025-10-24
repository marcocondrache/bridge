use std::sync::{Arc, Mutex};

use gpui::{Axis, Entity};

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
