use gpui::*;
use gpui_component::theme::ActiveTheme;

/// Semantic variants that communicate intent through visual appearance.
///
/// These variants map user intentions (primary action, destructive operation, etc.)
/// to consistent visual representations across all components.
///
/// # Variants
/// - **Default**: Neutral, standard appearance for normal actions
/// - **Primary**: Main action, uses brand/primary color
/// - **Secondary**: Supporting action, less emphasized than primary
/// - **Destructive**: Dangerous actions (delete, remove), uses destructive color
/// - **Ghost**: Minimal appearance, transparent background
/// - **Outline**: Bordered appearance, no background fill
#[derive(Clone, Default, Copy, PartialEq, Eq, Debug)]
pub enum Semantic {
    #[default]
    Default,
    Primary,
    Secondary,
    Destructive,
    Ghost,
    Outline,
}

impl Semantic {
    /// Returns the background color for this semantic variant.
    pub fn background(self, cx: &App) -> Hsla {
        let theme = cx.theme();

        match self {
            Self::Default => theme.background,
            Self::Primary => theme.primary,
            Self::Secondary => theme.secondary,
            Self::Destructive => theme.muted, // Using muted as fallback since destructive doesn't exist
            Self::Ghost | Self::Outline => hsla(0.0, 0.0, 0.0, 0.0),
        }
    }

    /// Returns the text/foreground color for this semantic variant.
    pub fn foreground(self, cx: &App) -> Hsla {
        let theme = cx.theme();

        match self {
            Self::Default => theme.foreground,
            Self::Primary | Self::Secondary | Self::Destructive => {
                hsla(0.0, 0.0, 1.0, 1.0) // white
            }
            Self::Ghost | Self::Outline => theme.foreground,
        }
    }

    /// Returns the border color for this semantic variant, if any.
    pub fn border(self, cx: &App) -> Option<Hsla> {
        let theme = cx.theme();

        match self {
            Self::Outline => Some(theme.border),
            Self::Ghost => None,
            Self::Default | Self::Primary | Self::Secondary | Self::Destructive => None,
        }
    }

    /// Returns the hover background color for this semantic variant.
    pub fn hover_background(self, cx: &App) -> Hsla {
        let theme = cx.theme();

        match self {
            Self::Default => theme.background,
            Self::Primary => {
                let color = theme.primary;
                Self::adjust_brightness(color, 0.9)
            }
            Self::Secondary => {
                let color = theme.secondary;
                Self::adjust_brightness(color, 0.9)
            }
            Self::Destructive => {
                let color = theme.muted;
                Self::adjust_brightness(color, 0.9)
            }
            Self::Ghost => theme.muted,
            Self::Outline => theme.muted,
        }
    }

    /// Adjusts the brightness of a color by a factor.
    fn adjust_brightness(color: Hsla, factor: f32) -> Hsla {
        hsla(color.h, color.s, color.l * factor, color.a)
    }

    /// Returns whether this variant should have a border by default.
    pub fn has_border(self) -> bool {
        matches!(self, Self::Outline | Self::Default)
    }
}
