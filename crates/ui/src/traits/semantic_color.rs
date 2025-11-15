use crate::variants::Semantic;

/// Trait for components that support semantic variants.
///
/// # Example
/// ```
/// use ui::prelude::*;
///
/// Button::new("delete")
///     .label("Delete")
///     .destructive()  // Red, indicates dangerous action
/// ```
pub trait SemanticColor: Sized {
    /// Set the semantic variant for this component.
    fn semantic_variant(self, variant: Semantic) -> Self;

    /// Use the default neutral appearance.
    fn default_variant(self) -> Self {
        self.semantic_variant(Semantic::Default)
    }

    /// Use the primary brand color (main actions).
    fn primary(self) -> Self {
        self.semantic_variant(Semantic::Primary)
    }

    /// Use the secondary color (supporting actions).
    fn secondary(self) -> Self {
        self.semantic_variant(Semantic::Secondary)
    }

    /// Use the destructive color (dangerous actions).
    fn destructive(self) -> Self {
        self.semantic_variant(Semantic::Destructive)
    }

    /// Use minimal ghost appearance (transparent background).
    fn ghost(self) -> Self {
        self.semantic_variant(Semantic::Ghost)
    }

    /// Use outline appearance (bordered, no fill).
    fn outline(self) -> Self {
        self.semantic_variant(Semantic::Outline)
    }
}
