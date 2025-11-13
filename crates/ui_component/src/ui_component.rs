use std::{collections::HashMap, sync::LazyLock};

use gpui::{AnyElement, App, SharedString, Window};
use parking_lot::RwLock;

pub static COMPONENTS: LazyLock<RwLock<ComponentRegistry>> =
    LazyLock::new(|| RwLock::new(ComponentRegistry::default()));

pub fn init() {
    for f in inventory::iter::<ComponentFn>() {
        (f.0)();
    }
}

pub struct ComponentFn(fn());

impl ComponentFn {
    pub const fn new(f: fn()) -> Self {
        Self(f)
    }
}

inventory::collect!(ComponentFn);

pub fn registry() -> ComponentRegistry {
    COMPONENTS.read().clone()
}

pub fn register_component<T: Component>() {
    let id = T::id();
    let entry = ComponentEntry {
        id: id.clone(),
        name: SharedString::new_static(T::name()),
        description: T::description().map(Into::into),
        story: Some(T::story),
    };

    let mut guard = COMPONENTS.write();
    guard.components.insert(id, entry);
}

/// Private internals for macros.
#[doc(hidden)]
pub mod __private {
    pub use inventory;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComponentId(&'static str);

#[derive(Default, Clone)]
pub struct ComponentRegistry {
    components: HashMap<ComponentId, ComponentEntry>,
}

impl ComponentRegistry {
    pub fn component_map(&self) -> HashMap<ComponentId, ComponentEntry> {
        self.components.clone()
    }
}

#[derive(Clone)]
pub struct ComponentEntry {
    id: ComponentId,
    name: SharedString,
    description: Option<SharedString>,
    story: Option<fn(&mut Window, &mut App) -> Option<AnyElement>>,
}

impl ComponentEntry {
    pub fn id(&self) -> ComponentId {
        self.id.clone()
    }

    pub fn name(&self) -> &SharedString {
        &self.name
    }

    pub fn description(&self) -> Option<&SharedString> {
        self.description.as_ref()
    }

    pub fn story(&self) -> Option<fn(&mut Window, &mut App) -> Option<AnyElement>> {
        self.story
    }
}

pub trait Component {
    fn id() -> ComponentId {
        ComponentId(Self::name())
    }

    fn name() -> &'static str {
        std::any::type_name::<Self>()
    }

    fn description() -> Option<&'static str> {
        None
    }

    fn story(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        None
    }
}
