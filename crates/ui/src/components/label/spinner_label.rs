use std::time::Duration;

use gpui::{Animation, AnimationExt, IntoElement, RenderOnce};

use crate::{components::label::Label, traits::SemanticColor};

const FRAMES: [&'static str; 8] = ["◐", "◓", "◑", "◒", "◐", "◓", "◑", "◒"];
const DURATION: Duration = Duration::from_millis(600);

#[derive(IntoElement)]
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
}

impl SemanticColor for SpinnerLabel {
    fn semantic_variant(mut self, variant: crate::prelude::Semantic) -> Self {
        self.base = self.base.semantic_variant(variant);
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
