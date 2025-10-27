use std::{
    any::{Any, TypeId, type_name},
    collections::{HashMap, hash_map::Entry},
};

use anyhow::{Ok, Result};
use config::{ConfigBuilder, builder::AsyncState};
use gpui::{App, AsyncApp, Global, UpdateGlobal};

use crate::settings_content::SettingsContent;

pub trait Settings: 'static + Send + Sync + Sized {
    fn from_settings(content: &SettingsContent) -> Self;

    #[track_caller]
    fn register(cx: &mut App)
    where
        Self: Sized,
    {
        SettingsStore::update_global(cx, |store, _| {
            store.register::<Self>();
        });
    }

    fn get<'a>(cx: &'a App) -> &'a Self
    where
        Self: Sized,
    {
        cx.global::<SettingsStore>().get()
    }

    fn get_global(cx: &App) -> &Self
    where
        Self: Sized,
    {
        cx.global::<SettingsStore>().get()
    }

    fn try_get(cx: &App) -> Option<&Self>
    where
        Self: Sized,
    {
        if cx.has_global::<SettingsStore>() {
            cx.global::<SettingsStore>().try_get()
        } else {
            None
        }
    }

    fn try_read_global<R>(cx: &AsyncApp, f: impl FnOnce(&Self) -> R) -> Option<R>
    where
        Self: Sized,
    {
        cx.try_read_global(|s: &SettingsStore, _| f(s.get()))
    }

    fn override_global(settings: Self, cx: &mut App)
    where
        Self: Sized,
    {
        cx.global_mut::<SettingsStore>().override_global(settings)
    }
}

pub trait SettingsHandle: 'static + Send + Sync {
    fn from_settings(&self, s: &SettingsContent) -> Box<dyn Any>;

    fn value(&self) -> &dyn Any;
}

impl<T: Settings> SettingsHandle for T {
    fn from_settings(&self, s: &SettingsContent) -> Box<dyn Any> {
        Box::new(T::from_settings(s))
    }

    fn value(&self) -> &dyn Any {
        self
    }
}

pub struct SettingsStore {
    shards: HashMap<TypeId, Box<dyn SettingsHandle>>,
    builder: ConfigBuilder<AsyncState>,
    snapshot: Option<SettingsContent>,
}

impl SettingsStore {
    pub fn new(builder: ConfigBuilder<AsyncState>) -> Self {
        Self {
            shards: HashMap::new(),
            builder,
            snapshot: None,
        }
    }

    pub fn register<T: Settings>(&mut self) {
        let entry = self.shards.entry(TypeId::of::<T>());

        match entry {
            Entry::Occupied(_) => {}
            Entry::Vacant(entry) => {
                let shard = T::from_settings(
                    self.snapshot
                        .as_ref()
                        .expect("snapshot must be registered when register is called"),
                );

                entry.insert(Box::new(shard));
            }
        }
    }

    pub fn get<T: Settings>(&self) -> &T {
        self.shards
            .get(&TypeId::of::<T>())
            .unwrap_or_else(|| panic!("unregistered setting type {}", type_name::<T>()))
            .value()
            .downcast_ref::<T>()
            .expect("no default value for setting type")
    }

    pub fn try_get<T: Settings>(&self) -> Option<&T> {
        self.shards
            .get(&TypeId::of::<T>())
            .map(|value| value.value())
            .and_then(|value| value.downcast_ref::<T>())
    }

    pub fn override_global<T: Settings>(&mut self, value: T) {
        let entry = self
            .shards
            .get_mut(&TypeId::of::<T>())
            .unwrap_or_else(|| panic!("unregistered setting type {}", type_name::<T>()));

        *entry = Box::new(value);
    }

    pub async fn reload(&mut self) -> Result<()> {
        let config = self.builder.build_cloned().await?;

        self.snapshot = Some(config.try_deserialize()?);

        Ok(())
    }
}

impl Global for SettingsStore {}
