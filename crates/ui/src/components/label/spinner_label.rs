use std::time::Duration;

use gpui::{Animation, AnimationExt, Hsla, IntoElement, RenderOnce, div};
use ui_component::{Component, titled_group, variant};
use ui_macros::RegisterComponent;

use crate::{components::label::Label, traits::Sizable};

const FRAMES: [&'static str; 8] = ["◐", "◓", "◑", "◒", "◐", "◓", "◑", "◒"];
const DURATION: Duration = Duration::from_millis(600);

#[derive(IntoElement, RegisterComponent)]
pub struct SpinnerLabel {
    base: Label,
    duration: Duration,
}

impl SpinnerLabel {
    pub fn new() -> Self {
        Self {
            base: Label::new(FRAMES[0]),
            duration: DURATION,
        }
    }

    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.base = self.base.color(color);
        self
    }
}

impl Sizable for SpinnerLabel {
    fn size(mut self, size: crate::prelude::Size) -> Self {
        self.base = self.base.size(size);
        self
    }
}

impl RenderOnce for SpinnerLabel {
    fn render(self, _: &mut gpui::Window, _: &mut gpui::App) -> impl gpui::IntoElement {
        self.base.with_animation(
            "spinner_label",
            Animation::new(self.duration).repeat(),
            move |mut label, delta| {
                let frame_index = (delta * FRAMES.len() as f32) as usize % FRAMES.len();

                label.set_text(FRAMES[frame_index]);
                label
            },
        )
    }
}

impl Component for SpinnerLabel {
    fn showcase(_window: &mut gpui::Window, cx: &mut gpui::App) -> Option<gpui::AnyElement> {
        use crate::prelude::*;
        use gpui_component::ActiveTheme;

        let theme = cx.theme();

        Some(
            div()
                .v_flex()
                .gap_6()
                .children(vec![
                    titled_group(
                        "Sizes",
                        vec![
                            variant("Small", SpinnerLabel::new().small().into_any_element()),
                            variant("Default", SpinnerLabel::new().into_any_element()),
                            variant("Medium", SpinnerLabel::new().medium().into_any_element()),
                            variant("Large", SpinnerLabel::new().large().into_any_element()),
                        ],
                    ),
                    titled_group(
                        "Colors",
                        vec![
                            variant("Default", SpinnerLabel::new().into_any_element()),
                            variant(
                                "Primary",
                                SpinnerLabel::new().color(theme.primary).into_any_element(),
                            ),
                            variant(
                                "Success",
                                SpinnerLabel::new().color(theme.success).into_any_element(),
                            ),
                            variant(
                                "Warning",
                                SpinnerLabel::new().color(theme.warning).into_any_element(),
                            ),
                            variant(
                                "Danger",
                                SpinnerLabel::new().color(theme.danger).into_any_element(),
                            ),
                        ],
                    ),
                ])
                .into_any_element(),
        )
    }
}
