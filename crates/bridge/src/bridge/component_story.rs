use std::collections::HashMap;

use gpui::{
    Context, FocusHandle, Focusable, InteractiveElement, IntoElement, ParentElement, Render,
    StatefulInteractiveElement, Styled, Window, div, prelude::FluentBuilder,
};
use gpui_component::ActiveTheme;
use ui::traits::styled_ext::StyledExt;
use ui_component::{ComponentEntry, ComponentId};
use workspace::area::Item;

pub struct ComponentStory {
    active_component: Option<ComponentId>,
    component_map: HashMap<ComponentId, ComponentEntry>,
    focus_handle: FocusHandle,
}

impl ComponentStory {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let registry = ui_component::registry();
        let component_map = registry.component_map();

        Self {
            component_map,
            active_component: None,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn render_component_page(
        &self,
        id: &ComponentId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let component = self.component_map.get(id);

        if let Some(component) = component {
            let child = match component.story() {
                Some(story) => story(window, cx).unwrap_or_else(|| {
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

impl Render for ComponentStory {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let theme = cx.theme();

        div()
            .h_flex()
            .id("component-story")
            .key_context("ComponentPreview")
            .items_start()
            .overflow_hidden()
            .size_full()
            .track_focus(&self.focus_handle)
            .bg(theme.background)
            .child(div())
            .child(
                div()
                    .v_flex()
                    .id("content-area")
                    .flex_1()
                    .size_full()
                    .overflow_y_scroll()
                    .when_some(self.active_component.clone(), |parent, id| {
                        parent.child(self.render_component_page(&id, window, cx))
                    }),
            )
    }
}

impl Focusable for ComponentStory {
    fn focus_handle(&self, _cx: &gpui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for ComponentStory {
    fn tab_title(&self, cx: &gpui::App) -> gpui::SharedString {
        "Component Story".into()
    }
}
