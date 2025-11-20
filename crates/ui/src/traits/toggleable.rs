pub trait Toggleable {
    fn toggle_state(self, selected: bool) -> Self;
}
