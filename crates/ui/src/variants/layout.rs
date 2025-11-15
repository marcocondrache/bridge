use gpui::*;

/// Layout variants control spacing and arrangement of components.
///
/// These variants affect how components integrate with their surrounding
/// layout and can be used to create different visual densities.
///
/// # Variants
/// - **Standalone**: Default margins, standard spacing
/// - **Compact**: Reduced spacing, for dense UIs
/// - **Inline**: No vertical margins, for inline elements
/// - **Block**: Full width, block-level display
#[derive(Clone, Default, Copy, PartialEq, Eq, Debug)]
pub enum Layout {
    #[default]
    Standalone,
    Compact,
    Inline,
    Block,
}

impl Layout {
    /// Returns the margin for this layout variant.
    pub fn margin(self) -> Pixels {
        match self {
            Self::Standalone => px(8.0),
            Self::Compact => px(4.0),
            Self::Inline => px(0.0),
            Self::Block => px(8.0),
        }
    }

    /// Returns the vertical margin for this layout variant.
    pub fn margin_y(self) -> Pixels {
        match self {
            Self::Standalone => px(4.0),
            Self::Compact => px(2.0),
            Self::Inline => px(0.0),
            Self::Block => px(4.0),
        }
    }

    /// Returns the horizontal margin for this layout variant.
    pub fn margin_x(self) -> Pixels {
        match self {
            Self::Standalone => px(4.0),
            Self::Compact => px(2.0),
            Self::Inline => px(2.0),
            Self::Block => px(0.0),
        }
    }

    /// Returns whether this variant should take full width.
    pub fn is_full_width(self) -> bool {
        matches!(self, Self::Block)
    }

    /// Returns whether this variant is inline.
    pub fn is_inline(self) -> bool {
        matches!(self, Self::Inline)
    }
}

/// Spacing scale for consistent gaps and padding.
///
/// Provides a standardized set of spacing values based on the 8px grid system.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Spacing {
    /// 4px - Extra small spacing
    XSmall,
    /// 8px - Small spacing
    Small,
    /// 12px - Default spacing
    Default,
    /// 16px - Medium spacing
    Medium,
    /// 24px - Large spacing
    Large,
    /// 32px - Extra large spacing
    XLarge,
}

impl Spacing {
    /// Returns the pixel value for this spacing.
    pub fn px(self) -> Pixels {
        match self {
            Self::XSmall => px(4.0),
            Self::Small => px(8.0),
            Self::Default => px(12.0),
            Self::Medium => px(16.0),
            Self::Large => px(24.0),
            Self::XLarge => px(32.0),
        }
    }
}
