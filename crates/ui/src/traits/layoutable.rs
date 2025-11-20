use crate::variants::Layout;

/// Trait for components that support layout variants.
///
/// # Example
/// ```
/// use ui::prelude::*;
///
/// Button::new("submit")
///     .label("Submit")
///     .block()  // Full width
/// ```
pub trait Layoutable: Sized {
    /// Set the layout variant for this component.
    fn layout_variant(self, variant: Layout) -> Self;

    /// Use standalone layout with default margins.
    fn standalone(self) -> Self {
        self.layout_variant(Layout::Standalone)
    }

    /// Use compact layout with reduced spacing.
    fn compact(self) -> Self {
        self.layout_variant(Layout::Compact)
    }

    /// Use inline layout with no vertical margins.
    fn inline_layout(self) -> Self {
        self.layout_variant(Layout::Inline)
    }

    /// Use block layout (full width).
    fn block(self) -> Self {
        self.layout_variant(Layout::Block)
    }
}
