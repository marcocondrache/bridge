use gpui::AnyView;

pub trait Item {}

pub trait ItemHandle {
    fn to_any(&self) -> AnyView;
}
