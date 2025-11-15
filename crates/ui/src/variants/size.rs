use gpui::*;

/// Size variants for components following an 8px base unit scale.
///
/// The size system ensures pixel-perfect alignment and consistent
/// spacing across all components.
///
/// # Scale
/// - Small: 16px (2 units) - Compact UIs, dense tables
/// - Default: 24px (3 units) - Standard UI elements
/// - Medium: 32px (4 units) - Emphasized controls
/// - Large: 40px (5 units) - Primary CTAs, hero elements
#[derive(Clone, Default, Copy, PartialEq, Eq, Debug)]
pub enum Size {
    Small,
    #[default]
    Default,
    Medium,
    Large,
}

impl Size {
    /// Returns the standard height for this size variant.
    ///
    /// Based on 8px base unit:
    /// - Small: 16px (2 units)
    /// - Default: 24px (3 units)
    /// - Medium: 32px (4 units)
    /// - Large: 40px (5 units)
    pub fn height(self) -> Pixels {
        match self {
            Size::Small => px(16.0),
            Size::Default => px(24.0),
            Size::Medium => px(32.0),
            Size::Large => px(40.0),
        }
    }

    /// Returns the horizontal padding for this size variant.
    pub fn padding_x(self) -> Pixels {
        match self {
            Size::Small => px(8.0),
            Size::Default => px(12.0),
            Size::Medium => px(16.0),
            Size::Large => px(20.0),
        }
    }

    /// Returns the vertical padding for this size variant.
    pub fn padding_y(self) -> Pixels {
        match self {
            Size::Small => px(2.0),
            Size::Default => px(4.0),
            Size::Medium => px(6.0),
            Size::Large => px(8.0),
        }
    }

    /// Returns the appropriate font size for this size variant.
    pub fn font_size(self) -> Pixels {
        match self {
            Size::Small => px(11.0),
            Size::Default => px(13.0),
            Size::Medium => px(14.0),
            Size::Large => px(16.0),
        }
    }

    /// Returns the gap spacing for this size variant.
    ///
    /// Used for spacing between elements within a component.
    pub fn gap(self) -> Pixels {
        match self {
            Size::Small => px(4.0),
            Size::Default => px(6.0),
            Size::Medium => px(8.0),
            Size::Large => px(12.0),
        }
    }

    /// Returns the icon size for this size variant.
    pub fn icon_size(self) -> Pixels {
        match self {
            Size::Small => px(12.0),
            Size::Default => px(14.0),
            Size::Medium => px(16.0),
            Size::Large => px(20.0),
        }
    }
}
