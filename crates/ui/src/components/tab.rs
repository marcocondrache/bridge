use gpui::{
    AnyElement, App, Div, ElementId, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    Styled, Window, div, prelude::FluentBuilder,
};
use gpui_component::{ActiveTheme, v_flex};
use smallvec::SmallVec;
use ui_component::{Component, titled_group, variant};
use ui_macros::RegisterComponent;

use crate::prelude::*;

#[derive(IntoElement, RegisterComponent)]
pub struct Tab {
    id: ElementId,
    base: Div,
    selected: bool,
    disabled: bool,
    size: Size,
    start_slot: Option<AnyElement>,
    end_slot: Option<AnyElement>,
    children: SmallVec<[AnyElement; 2]>,
}

impl Tab {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            base: div(),
            selected: false,
            disabled: false,
            size: Size::default(),
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

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl InteractiveElement for Tab {
    fn interactivity(&mut self) -> &mut gpui::Interactivity {
        self.base.interactivity()
    }
}

impl StatefulInteractiveElement for Tab {}

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

        self.base
            .h_flex()
            .items_center()
            .justify_center()
            .gap(gap)
            .h(height)
            .px(padding_x)
            .py(padding_y)
            .text_size(font_size)
            .border_r_1()
            .border_color(theme.border)
            .map(|this| match self.selected {
                true => this
                    .text_color(theme.tab_active_foreground)
                    .bg(theme.tab_active),
                false => this.text_color(theme.tab_foreground).bg(theme.tab),
            })
            .when_else(
                self.disabled,
                |this| {
                    this.opacity(0.4)
                        .cursor(gpui::CursorStyle::OperationNotAllowed)
                },
                |this| this.cursor(gpui::CursorStyle::PointingHand),
            )
            .when(!self.selected && !self.disabled, |this| {
                this.hover(|style| style.bg(theme.secondary_hover))
            })
            .when_some(self.start_slot, |this, slot| this.child(slot))
            .children(self.children)
            .when_some(self.end_slot, |this, slot| this.child(slot))
            .id(self.id)
    }
}

impl Component for Tab {
    fn showcase(_window: &mut Window, _: &mut App) -> Option<AnyElement> {
        Some(
            v_flex()
                .gap_6()
                .children(vec![
                    titled_group(
                        "Sizes",
                        vec![
                            variant(
                                "Small",
                                div()
                                    .h_flex()
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
                                div()
                                    .h_flex()
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
                                div()
                                    .h_flex()
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
                                div()
                                    .h_flex()
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
