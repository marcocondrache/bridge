use gpui::Axis;

#[derive(Debug, Clone, Copy)]
pub enum Placement {
    Left,
    Right,
    Top,
    Bottom,
}

impl Placement {
    pub fn opposite(&self) -> Self {
        match self {
            Placement::Left => Placement::Right,
            Placement::Right => Placement::Left,
            Placement::Top => Placement::Bottom,
            Placement::Bottom => Placement::Top,
        }
    }

    pub fn axis(&self) -> Axis {
        match self {
            Placement::Left | Placement::Right => Axis::Horizontal,
            Placement::Top | Placement::Bottom => Axis::Vertical,
        }
    }
}
