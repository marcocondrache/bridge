mod colors;
mod scales;

use std::sync::Arc;

use derive_more::{Deref, DerefMut};
use gpui::{App, BorrowAppContext, Global, WindowAppearance};

use crate::colors::ThemeColors;

pub fn init(cx: &mut App) {
    SystemAppearance::init(cx);

    let default_theme = Theme {
        colors: ThemeColors::dark(),
        appearance: ThemeAppearance::Dark,
    };

    cx.set_global(GlobalTheme {
        theme: Arc::new(default_theme),
    });
}

pub trait ActiveTheme {
    fn theme(&self) -> &Arc<Theme>;
}

impl ActiveTheme for App {
    fn theme(&self) -> &Arc<Theme> {
        GlobalTheme::theme(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThemeAppearance {
    Light,
    Dark,
}

impl ThemeAppearance {
    pub fn is_dark(&self) -> bool {
        matches!(self, ThemeAppearance::Dark)
    }

    pub fn is_light(&self) -> bool {
        matches!(self, ThemeAppearance::Light)
    }
}

impl From<WindowAppearance> for ThemeAppearance {
    fn from(appearance: WindowAppearance) -> Self {
        match appearance {
            WindowAppearance::Light | WindowAppearance::VibrantLight => ThemeAppearance::Light,
            WindowAppearance::Dark | WindowAppearance::VibrantDark => ThemeAppearance::Dark,
        }
    }
}

#[derive(Debug, Clone, Copy, Deref)]
pub struct SystemAppearance(pub ThemeAppearance);

impl Default for SystemAppearance {
    fn default() -> Self {
        Self(ThemeAppearance::Dark)
    }
}

#[derive(Default, Deref, DerefMut)]
struct GlobalSystemAppearance(SystemAppearance);

impl Global for GlobalSystemAppearance {}

impl SystemAppearance {
    pub fn init(cx: &mut App) {
        *cx.default_global::<GlobalSystemAppearance>() =
            GlobalSystemAppearance(SystemAppearance(cx.window_appearance().into()));
    }

    pub fn global(cx: &App) -> Self {
        cx.global::<GlobalSystemAppearance>().0
    }

    pub fn global_mut(cx: &mut App) -> &mut Self {
        cx.global_mut::<GlobalSystemAppearance>()
    }
}

pub struct Theme {
    pub colors: ThemeColors,
    pub appearance: ThemeAppearance,
}

impl Theme {
    #[inline]
    pub fn colors(&self) -> &ThemeColors {
        &self.colors
    }

    #[inline]
    pub fn appearance(&self) -> ThemeAppearance {
        self.appearance
    }
}

pub struct GlobalTheme {
    theme: Arc<Theme>,
}

impl Global for GlobalTheme {}

impl GlobalTheme {
    pub fn reload_theme(cx: &mut App) {
        let system = SystemAppearance::global(cx);

        let colors = match system.0 {
            ThemeAppearance::Light => ThemeColors::light(),
            ThemeAppearance::Dark => ThemeColors::dark(),
        };

        let theme = Theme {
            colors,
            appearance: system.0,
        };

        cx.update_global::<Self, _>(|this, _| this.theme = Arc::new(theme));
        cx.refresh_windows();
    }

    pub fn theme(cx: &App) -> &Arc<Theme> {
        &cx.global::<Self>().theme
    }
}
