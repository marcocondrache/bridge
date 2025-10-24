use crate::item::ItemHandle;

pub struct Pane {
    items: Vec<Box<dyn ItemHandle>>,
}

impl Pane {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
}
