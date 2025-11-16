use gpui::{Div, FontWeight, Hsla, SharedString, UnderlineStyle, div, px};
use gpui_component::ActiveTheme;
use ui_component::{Component, titled_group, variant};
use ui_macros::RegisterComponent;

use crate::prelude::*;

#[derive(IntoElement, RegisterComponent)]
pub struct Label {
    base: Div,
    size: Size,
    label: SharedString,
    weight: Option<FontWeight>,
    color: Option<Hsla>,
    truncate: bool,
    italic: bool,
    underline: bool,
    strikethrough: bool,
}

impl Label {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            base: div(),
            size: Size::default(),
            label: label.into(),
            weight: None,
            color: None,
            truncate: false,
            italic: false,
            underline: false,
            strikethrough: false,
        }
    }

    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn strikethrough(mut self) -> Self {
        self.strikethrough = true;
        self
    }

    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }

    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }

    pub fn weight(mut self, weight: gpui::FontWeight) -> Self {
        self.weight = Some(weight);
        self
    }

    pub fn set_text(&mut self, text: impl Into<SharedString>) {
        self.label = text.into();
    }

    pub fn truncate(mut self, truncate: bool) -> Self {
        self.truncate = truncate;
        self
    }
}

impl Sizable for Label {
    fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
}

impl ParentElement for Label {
    fn extend(&mut self, elements: impl IntoIterator<Item = gpui::AnyElement>) {
        self.base.extend(elements);
    }
}

impl RenderOnce for Label {
    fn render(self, _: &mut gpui::Window, cx: &mut gpui::App) -> impl gpui::IntoElement {
        let theme = cx.theme();
        let text_size = self.size.font_size();

        self.base
            .text_size(text_size)
            .text_color(self.color.unwrap_or(theme.foreground))
            .when_some(self.weight, |this, weight| this.font_weight(weight))
            .when(self.italic, |this| this.italic())
            .when(self.strikethrough, |this| this.line_through())
            .when(self.truncate, |this| {
                this.overflow_x_hidden().text_ellipsis()
            })
            .when(self.underline, |mut this| {
                this.text_style()
                    .get_or_insert_with(Default::default)
                    .underline = Some(UnderlineStyle {
                    thickness: px(1.),
                    color: None,
                    wavy: false,
                });
                this
            })
            .child(self.label)
    }
}

impl Component for Label {
    fn showcase(_window: &mut gpui::Window, cx: &mut gpui::App) -> Option<gpui::AnyElement> {
        let theme = cx.theme();

        Some(
            div()
                .v_flex()
                .gap_6()
                .children(vec![
                    titled_group(
                        "Sizes",
                        vec![
                            variant(
                                "Small",
                                Label::new("Small label").small().into_any_element(),
                            ),
                            variant(
                                "Default",
                                Label::new("Default label").into_any_element(),
                            ),
                            variant(
                                "Medium",
                                Label::new("Medium label").medium().into_any_element(),
                            ),
                            variant(
                                "Large",
                                Label::new("Large label").large().into_any_element(),
                            ),
                        ],
                    ),
                    titled_group(
                        "Font Weights",
                        vec![
                            variant(
                                "Normal",
                                Label::new("Normal weight")
                                    .weight(FontWeight::NORMAL)
                                    .into_any_element(),
                            ),
                            variant(
                                "Medium",
                                Label::new("Medium weight")
                                    .weight(FontWeight::MEDIUM)
                                    .into_any_element(),
                            ),
                            variant(
                                "Semibold",
                                Label::new("Semibold weight")
                                    .weight(FontWeight::SEMIBOLD)
                                    .into_any_element(),
                            ),
                            variant(
                                "Bold",
                                Label::new("Bold weight")
                                    .weight(FontWeight::BOLD)
                                    .into_any_element(),
                            ),
                        ],
                    ),
                    titled_group(
                        "Text Decorations",
                        vec![
                            variant(
                                "Normal",
                                Label::new("Normal text").into_any_element(),
                            ),
                            variant(
                                "Italic",
                                Label::new("Italic text").italic().into_any_element(),
                            ),
                            variant(
                                "Underline",
                                Label::new("Underlined text").underline().into_any_element(),
                            ),
                            variant(
                                "Strikethrough",
                                Label::new("Strikethrough text")
                                    .strikethrough()
                                    .into_any_element(),
                            ),
                            variant(
                                "Combined",
                                Label::new("Italic, underlined, and bold")
                                    .italic()
                                    .underline()
                                    .weight(FontWeight::BOLD)
                                    .into_any_element(),
                            ),
                        ],
                    ),
                    titled_group(
                        "Colors",
                        vec![
                            variant(
                                "Default",
                                Label::new("Default color").into_any_element(),
                            ),
                            variant(
                                "Primary",
                                Label::new("Primary color")
                                    .color(theme.primary)
                                    .into_any_element(),
                            ),
                            variant(
                                "Success",
                                Label::new("Success color")
                                    .color(theme.success)
                                    .into_any_element(),
                            ),
                            variant(
                                "Warning",
                                Label::new("Warning color")
                                    .color(theme.warning)
                                    .into_any_element(),
                            ),
                            variant(
                                "Danger",
                                Label::new("Danger color")
                                    .color(theme.danger)
                                    .into_any_element(),
                            ),
                        ],
                    ),
                    titled_group(
                        "Truncation",
                        vec![
                            variant(
                                "Normal",
                                div()
                                    .w_64()
                                    .child(
                                        Label::new("This is a very long label that will wrap to multiple lines when it exceeds the container width")
                                            .into_any_element()
                                    )
                                    .into_any_element(),
                            ),
                            variant(
                                "Truncated",
                                div()
                                    .w_64()
                                    .child(
                                        Label::new("This is a very long label that will be truncated with an ellipsis when it exceeds the container width")
                                            .truncate(true)
                                            .into_any_element()
                                    )
                                    .into_any_element(),
                            ),
                        ],
                    ),
                ])
                .into_any_element(),
        )
    }
}
