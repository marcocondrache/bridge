use std::{collections::HashMap, ops::Range};

use gpui::{
    ClickEvent, Context, FocusHandle, Focusable, InteractiveElement, IntoElement, ParentElement,
    Render, StatefulInteractiveElement, Styled, Window, div, prelude::FluentBuilder, px,
};
use gpui_component::ActiveTheme;
use ui::traits::styled_ext::StyledExt;
use ui_component::{ComponentId, RegisteredComponent};
use workspace::area::Item;

pub struct ComponentShowcase {
    active_component: Option<ComponentId>,
    component_map: HashMap<ComponentId, RegisteredComponent>,
    focus_handle: FocusHandle,
}

impl ComponentShowcase {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let registry = ui_component::registry();
        let component_map = registry.component_map();

        Self {
            component_map,
            active_component: None,
            focus_handle: cx.focus_handle(),
        }
    }

    fn sidebar_entries(&self) -> Vec<ComponentId> {
        self.component_map.keys().cloned().collect()
    }

    fn render_sidebar_entry(
        &self,
        id: ComponentId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + use<> {
        let component = self.component_map.get(&id);

        if let Some(component) = component {
            div()
                .id("ccc")
                .on_click(cx.listener({
                    move |this, _: &ClickEvent, _, _| this.active_component = Some(id)
                }))
                .child(component.scopeless_name().clone())
                .into_any_element()
        } else {
            div().into_any_element()
        }
    }

    fn render_component_page(
        &self,
        id: ComponentId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let component = self.component_map.get(&id);

        if let Some(component) = component {
            let child = match component.showcase() {
                Some(showcase) => showcase(window, cx).unwrap_or_else(|| {
                    div()
                        .child("Failed to load preview. This path should be unreachable")
                        .into_any_element()
                }),
                None => div().child("No preview available").into_any_element(),
            };

            div()
                .v_flex()
                .size_full()
                .flex_1()
                .px_12()
                .py_6()
                .child(child)
        } else {
            div()
                .v_flex()
                .size_full()
                .items_center()
                .justify_center()
                .child("Component not found")
        }
    }
}

impl Render for ComponentShowcase {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let theme = cx.theme();
        let sidebar_entries = self.sidebar_entries();

        div()
            .h_flex()
            .id("component-showcase")
            .key_context("ComponentPreview")
            .items_start()
            .overflow_hidden()
            .size_full()
            .track_focus(&self.focus_handle)
            .bg(theme.background)
            .child(
                div()
                    .v_flex()
                    .h_full()
                    .border_r_1()
                    .border_color(theme.border)
                    .child(
                        gpui::uniform_list(
                            "component-nav",
                            sidebar_entries.len(),
                            cx.processor(move |this, range: Range<usize>, window, cx| {
                                range
                                    .filter_map(|ix| {
                                        if ix < sidebar_entries.len() {
                                            Some(this.render_sidebar_entry(
                                                sidebar_entries[ix].clone(),
                                                window,
                                                cx,
                                            ))
                                        } else {
                                            None
                                        }
                                    })
                                    .collect()
                            }),
                        )
                        .p_2p5()
                        .w(px(231.))
                        .h_full()
                        .flex_1(),
                    ),
            )
            .child(
                div()
                    .v_flex()
                    .id("content-area")
                    .flex_1()
                    .size_full()
                    .overflow_y_scroll()
                    .when_some(self.active_component.clone(), |parent, id| {
                        parent.child(self.render_component_page(id, window, cx))
                    }),
            )
    }
}

impl Focusable for ComponentShowcase {
    fn focus_handle(&self, _cx: &gpui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for ComponentShowcase {
    fn tab_title(&self, cx: &gpui::App) -> gpui::SharedString {
        "Component Showcase".into()
    }
}
