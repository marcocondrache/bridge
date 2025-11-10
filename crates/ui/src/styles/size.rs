#[derive(Clone, Default, Copy, PartialEq, Eq, Debug)]
pub enum Size {
    Large,
    Medium,
    #[default]
    Default,
}

pub trait Sizable: Sized {
    fn with_size(self, size: Size) -> Self;

    fn default(self) -> Self {
        self.with_size(Size::Default)
    }

    fn medium(self) -> Self {
        self.with_size(Size::Medium)
    }

    fn large(self) -> Self {
        self.with_size(Size::Large)
    }
}
