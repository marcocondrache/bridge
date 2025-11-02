use std::borrow::Cow;

use config::{ConfigBuilder, builder::AsyncState};
use gpui::App;
use rust_embed::Embed as RustEmbed;
use util::asset_str;

use crate::settings_store::SettingsStore;

mod settings_content;
mod settings_store;

pub use settings_content::SettingsContent;
pub use settings_store::Settings;

pub fn init(cx: &mut App) {
    let builder = ConfigBuilder::<AsyncState>::default().add_source(config::File::from_str(
        &default_settings(),
        config::FileFormat::Json5,
    ));

    cx.set_global(SettingsStore::new(builder));
}

#[derive(RustEmbed)]
#[folder = "../../assets"]
#[include = "settings/*"]
#[exclude = "*.DS_Store"]
pub struct SettingsAssets;

pub fn default_settings() -> Cow<'static, str> {
    asset_str::<SettingsAssets>("settings/default.json")
}
