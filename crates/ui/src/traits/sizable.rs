use crate::variants::Size;

/// Trait for components that support size variants.
///
/// # Example
/// ```
/// use ui::prelude::*;
///
/// Button::new("submit")
///     .label("Submit")
///     .large()  // Convenience method
/// ```
pub trait Sizable: Sized {
    /// Set the size variant for this component.
    fn size(self, size: Size) -> Self;

    /// Set size to Small (16px height).
    fn small(self) -> Self {
        self.size(Size::Small)
    }

    /// Set size to Default (24px height).
    fn default_size(self) -> Self {
        self.size(Size::Default)
    }

    /// Set size to Medium (32px height).
    fn medium(self) -> Self {
        self.size(Size::Medium)
    }

    /// Set size to Large (40px height).
    fn large(self) -> Self {
        self.size(Size::Large)
    }
}
