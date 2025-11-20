/// Trait for components that have loading state.
pub trait Loadable: Sized {
    /// Set whether this component is in a loading state.
    fn loading(self, loading: bool) -> Self;
}
