use gpui::Axis;
use gpui_component::Placement as ExternalPlacement;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Placement {
    Top,
    Right,
    Bottom,
    Left,
}

impl Placement {
    pub fn opposite(&self) -> Self {
        match self {
            Placement::Top => Placement::Bottom,
            Placement::Right => Placement::Left,
            Placement::Bottom => Placement::Top,
            Placement::Left => Placement::Right,
        }
    }

    pub fn axis(&self) -> Axis {
        match self {
            Placement::Top | Placement::Bottom => Axis::Vertical,
            Placement::Left | Placement::Right => Axis::Horizontal,
        }
    }
}

// TODO: Until we migrate the entire components system
impl From<ExternalPlacement> for Placement {
    fn from(placement: ExternalPlacement) -> Self {
        match placement {
            ExternalPlacement::Top => Placement::Top,
            ExternalPlacement::Right => Placement::Right,
            ExternalPlacement::Bottom => Placement::Bottom,
            ExternalPlacement::Left => Placement::Left,
        }
    }
}

// TODO: Until we migrate the entire components system
impl From<Placement> for ExternalPlacement {
    fn from(placement: Placement) -> Self {
        match placement {
            Placement::Top => ExternalPlacement::Top,
            Placement::Right => ExternalPlacement::Right,
            Placement::Bottom => ExternalPlacement::Bottom,
            Placement::Left => ExternalPlacement::Left,
        }
    }
}
