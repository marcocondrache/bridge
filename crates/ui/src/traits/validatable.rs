use gpui::SharedString;

/// Trait for components that can display error/validation states.
pub trait Validatable: Sized {
    /// Set an error message for this component.
    fn error(self, error: Option<impl Into<SharedString>>) -> Self;

    /// Set whether this component is in a valid state.
    fn valid(self, valid: bool) -> Self {
        if valid {
            self.error(None::<&str>)
        } else {
            self
        }
    }
}
