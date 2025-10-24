use gpui::{Context, WeakEntity};

use crate::{Workspace, item::ItemHandle};

pub struct Pane {
    weak_self: WeakEntity<Pane>,
    items: Vec<Box<dyn ItemHandle>>,
    workspace: WeakEntity<Workspace>,
}

impl Pane {
    pub fn new(workspace: WeakEntity<Workspace>, cx: &mut Context<Self>) -> Self {
        let weak_self = cx.entity().downgrade();

        Self {
            weak_self,
            items: Vec::new(),
            workspace,
        }
    }
}
