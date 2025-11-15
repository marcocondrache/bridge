use gpui::{
    AnyElement, App, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce, Styled,
    Window, div, hsla, prelude::FluentBuilder, px, transparent_white,
};
use gpui_component::ActiveTheme;
use smallvec::SmallVec;
use ui_component::Component;
use ui_macros::RegisterComponent;

use crate::prelude::*;

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum TabStyle {
    #[default]
    Underline,
    Boxed,
}

#[derive(IntoElement, RegisterComponent)]
pub struct Tab {
    id: ElementId,
    selected: bool,
    disabled: bool,
    size: Size,
    tab_style: TabStyle,
    start_slot: Option<AnyElement>,
    end_slot: Option<AnyElement>,
    children: SmallVec<[AnyElement; 2]>,
}

impl Tab {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            selected: false,
            disabled: false,
            size: Size::default(),
            tab_style: TabStyle::default(),
            start_slot: None,
            end_slot: None,
            children: SmallVec::new(),
        }
    }

    pub fn start_slot<E: IntoElement>(mut self, element: impl Into<Option<E>>) -> Self {
        self.start_slot = element.into().map(IntoElement::into_any_element);
        self
    }

    pub fn end_slot<E: IntoElement>(mut self, element: impl Into<Option<E>>) -> Self {
        self.end_slot = element.into().map(IntoElement::into_any_element);
        self
    }

    pub fn underline(mut self) -> Self {
        self.tab_style = TabStyle::Underline;
        self
    }

    pub fn boxed(mut self) -> Self {
        self.tab_style = TabStyle::Boxed;
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl Disableable for Tab {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Sizable for Tab {
    fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
}

impl ParentElement for Tab {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for Tab {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

        let height = self.size.height();
        let padding_x = self.size.padding_x();
        let padding_y = self.size.padding_y();
        let gap = self.size.gap();
        let font_size = self.size.font_size();

        let (bg, fg, hover_bg, border_radius) = match (self.tab_style, self.selected) {
            (TabStyle::Underline, true) => (
                transparent_white(),
                theme.primary,
                theme.muted.alpha(0.3),
                px(0.0),
            ),
            (TabStyle::Underline, false) => (
                transparent_white(),
                theme.muted_foreground,
                theme.muted.alpha(0.2),
                px(0.0),
            ),
            (TabStyle::Boxed, true) => (
                theme.background,
                theme.foreground,
                theme.muted.alpha(0.2),
                theme.radius,
            ),
            (TabStyle::Boxed, false) => (
                theme.muted.alpha(0.2),
                theme.muted_foreground,
                theme.muted.alpha(0.3),
                theme.radius,
            ),
        };

        let indicator_height = match self.size {
            Size::Small | Size::Default => px(2.0),
            Size::Medium | Size::Large => px(3.0),
        };

        let tab_div = div()
            .h_flex()
            .items_center()
            .justify_center()
            .gap(gap)
            .h(height)
            .px(padding_x)
            .py(padding_y)
            .bg(bg)
            .text_color(fg)
            .text_size(font_size)
            .rounded(border_radius)
            .map(|this| match self.tab_style {
                TabStyle::Boxed if self.selected => this.border_1().border_color(theme.border),
                TabStyle::Underline if self.selected => {
                    this.font_weight(gpui::FontWeight::SEMIBOLD)
                }
                _ => this,
            })
            .when_else(
                self.disabled,
                |this| {
                    this.opacity(0.4)
                        .cursor(gpui::CursorStyle::OperationNotAllowed)
                },
                |this| {
                    this.cursor(gpui::CursorStyle::PointingHand).hover(|style| {
                        let styled = style.bg(hover_bg);
                        if self.tab_style == TabStyle::Underline && !self.selected {
                            styled.text_color(theme.foreground)
                        } else {
                            styled
                        }
                    })
                },
            )
            .when_some(self.start_slot, |this, slot| this.child(slot))
            .children(self.children)
            .when_some(self.end_slot, |this, slot| this.child(slot))
            .id(self.id);

        div().relative().child(tab_div).when(
            self.tab_style == TabStyle::Underline && self.selected,
            |this| {
                this.child(
                    div()
                        .absolute()
                        .bottom_0()
                        .left_0()
                        .right_0()
                        .h(indicator_height)
                        .bg(theme.primary)
                        .rounded_t(px(2.0)),
                )
            },
        )
    }
}

impl Component for Tab {
    fn showcase(_window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        use gpui_component::{h_flex, v_flex};
        use ui_component::{titled_group, variant};

        Some(
            v_flex()
                .gap_6()
                .children(vec![
                    titled_group(
                        "Tab Styles",
                        vec![
                            variant(
                                "Underline",
                                h_flex()
                                    .gap_1()
                                    .children(vec![
                                        Tab::new("u1")
                                            .child("Overview")
                                            .selected(true)
                                            .underline()
                                            .into_any_element(),
                                        Tab::new("u2")
                                            .child("Analytics")
                                            .underline()
                                            .into_any_element(),
                                        Tab::new("u3")
                                            .child("Settings")
                                            .underline()
                                            .into_any_element(),
                                    ])
                                    .into_any_element(),
                            ),
                            variant(
                                "Boxed",
                                h_flex()
                                    .gap_2()
                                    .children(vec![
                                        Tab::new("b1")
                                            .child("Dashboard")
                                            .selected(true)
                                            .boxed()
                                            .into_any_element(),
                                        Tab::new("b2").child("Reports").boxed().into_any_element(),
                                        Tab::new("b3").child("Users").boxed().into_any_element(),
                                    ])
                                    .into_any_element(),
                            ),
                        ],
                    ),
                    titled_group(
                        "Sizes",
                        vec![
                            variant(
                                "Small",
                                h_flex()
                                    .gap_2()
                                    .children(vec![
                                        Tab::new("s1")
                                            .child("Small")
                                            .selected(true)
                                            .small()
                                            .into_any_element(),
                                        Tab::new("s2").child("Tab").small().into_any_element(),
                                    ])
                                    .into_any_element(),
                            ),
                            variant(
                                "Default",
                                h_flex()
                                    .gap_2()
                                    .children(vec![
                                        Tab::new("d1")
                                            .child("Default")
                                            .selected(true)
                                            .into_any_element(),
                                        Tab::new("d2").child("Tab").into_any_element(),
                                    ])
                                    .into_any_element(),
                            ),
                            variant(
                                "Medium",
                                h_flex()
                                    .gap_2()
                                    .children(vec![
                                        Tab::new("m1")
                                            .child("Medium")
                                            .selected(true)
                                            .medium()
                                            .into_any_element(),
                                        Tab::new("m2").child("Tab").medium().into_any_element(),
                                    ])
                                    .into_any_element(),
                            ),
                            variant(
                                "Large",
                                h_flex()
                                    .gap_2()
                                    .children(vec![
                                        Tab::new("l1")
                                            .child("Large")
                                            .selected(true)
                                            .large()
                                            .into_any_element(),
                                        Tab::new("l2").child("Tab").large().into_any_element(),
                                    ])
                                    .into_any_element(),
                            ),
                        ],
                    ),
                    titled_group(
                        "States",
                        vec![
                            variant("Normal", Tab::new("n1").child("Normal").into_any_element()),
                            variant(
                                "Selected",
                                Tab::new("n2")
                                    .child("Selected")
                                    .selected(true)
                                    .into_any_element(),
                            ),
                            variant(
                                "Disabled",
                                Tab::new("n3")
                                    .child("Disabled")
                                    .disabled(true)
                                    .into_any_element(),
                            ),
                        ],
                    ),
                ])
                .into_any_element(),
        )
    }
}
