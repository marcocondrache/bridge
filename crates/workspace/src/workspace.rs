pub mod area;
pub mod dock;
pub mod item;

use std::sync::{Arc, Weak};

use anyhow::Ok;
use gpui::{
    Action, AnyView, App, AppContext, Context, Div, Entity, Global, InteractiveElement, KeyContext,
    ParentElement, Render, Styled, Subscription, Task, WeakEntity, Window, WindowHandle,
    WindowOptions, actions, div,
};
use gpui_component::{ActiveTheme, Placement, Root};
use uuid::Uuid;

use crate::{
    area::Area,
    dock::{Dock, Panel, PanelHandle},
    item::ItemHandle,
};

actions!(workspace, [NewHttpEditor, NewWindow]);

pub struct AppState {
    pub build_window_options: fn(Option<Uuid>, &mut App) -> WindowOptions,
}

struct GlobalAppState(Weak<AppState>);

impl Global for GlobalAppState {}

impl AppState {
    pub fn global(cx: &App) -> Weak<Self> {
        cx.global::<GlobalAppState>().0.clone()
    }

    pub fn try_global(cx: &App) -> Option<Weak<Self>> {
        cx.try_global::<GlobalAppState>()
            .map(|state| state.0.clone())
    }

    pub fn set_global(state: Weak<AppState>, cx: &mut App) {
        cx.set_global(GlobalAppState(state));
    }
}

pub struct Workspace {
    weak_self: WeakEntity<Self>,
    right_dock: Entity<Dock>,
    bottom_dock: Entity<Dock>,
    center: Entity<Area>,
    titlebar_item: Option<AnyView>,
    actions: Vec<Box<dyn Fn(Div, &Workspace, &mut Window, &mut Context<Self>) -> Div>>,
    _subscriptions: Vec<Subscription>,
}

impl Workspace {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let weak_self = cx.entity().downgrade();

        let right_dock = Dock::new(Placement::Right, cx);
        let bottom_dock = Dock::new(Placement::Bottom, cx);
        let center = Area::new(window, cx);

        let subscriptions = vec![];

        Self {
            weak_self,
            right_dock,
            bottom_dock,
            center,
            actions: Default::default(),
            titlebar_item: None,
            _subscriptions: subscriptions,
        }
    }

    pub fn add_panel<T: Panel>(
        &mut self,
        panel: Entity<T>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let placement = panel.placement(window, cx);
        let dock = self.dock_at_placement(placement);

        dock.update(cx, |dock, cx| {
            dock.add_panel(panel, cx);
        })
    }

    pub fn add_item(
        &mut self,
        item: Box<dyn ItemHandle>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.center.update(cx, |area, cx| {
            area.add_item(item, window, cx);
        });
    }

    pub fn register_action<A: Action>(
        &mut self,
        callback: impl Fn(&mut Self, &A, &mut Window, &mut Context<Self>) + 'static,
    ) -> &mut Self {
        let callback = Arc::new(callback);

        self.actions.push(Box::new(move |div, _, _, cx| {
            let callback = callback.clone();

            div.on_action(cx.listener(move |workspace, event, window, cx| {
                (callback)(workspace, event, window, cx)
            }))
        }));

        self
    }

    pub fn set_titlebar_item(&mut self, item: AnyView, _: &mut Window, cx: &mut Context<Self>) {
        self.titlebar_item = Some(item);
        cx.notify();
    }

    pub fn spawn(
        app_state: Arc<AppState>,
        cx: &mut App,
    ) -> Task<anyhow::Result<WindowHandle<Root>>> {
        let options = (app_state.build_window_options)(None, cx);

        cx.spawn(async move |cx| {
            let window = cx.open_window(options, {
                let _app_state = app_state.clone();

                move |window, cx| {
                    let workspace = cx.new(|cx| Workspace::new(window, cx));

                    cx.new(|cx| Root::new(workspace.into(), window, cx))
                }
            })?;

            window.update(cx, |_workspace, window, _cx| {
                window.activate_window();
            })?;

            Ok(window)
        })
    }

    fn actions(&self, div: Div, window: &mut Window, cx: &mut Context<Self>) -> Div {
        self.actions
            .iter()
            .fold(div, |div, action| (action)(div, self, window, cx))
    }

    fn dock_at_placement(&self, placement: Placement) -> &Entity<Dock> {
        match placement {
            Placement::Right => &self.right_dock,
            Placement::Bottom => &self.bottom_dock,
            _ => &self.right_dock,
        }
    }

    fn render_dock(&self, dock: &Entity<Dock>) -> Option<Div> {
        Some(
            div()
                .flex()
                .flex_none()
                .overflow_hidden()
                .child(dock.clone()),
        )
    }
}

pub fn open_new(app_state: Arc<AppState>, cx: &mut App) {
    let task = Workspace::spawn(app_state, cx);

    cx.spawn(async move |_| {
        let _ = task.await;
    })
    .detach();
}

impl Render for Workspace {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let theme = cx.theme().clone();
        let mut context = KeyContext::new_with_defaults();

        context.add("workspace");
        context.set("keyboard_layout", cx.keyboard_layout().name().to_string());

        self.actions(div(), window, cx)
            .key_context(context)
            .id("root")
            .relative()
            .size_full()
            .flex()
            .flex_col()
            .justify_start()
            .items_start()
            .text_color(theme.foreground)
            .overflow_hidden()
            .children(self.titlebar_item.clone())
            .child(
                div()
                    .id("workspace")
                    .bg(theme.background)
                    .relative()
                    .flex_1()
                    .w_full()
                    .flex()
                    .flex_col()
                    .overflow_hidden()
                    .border_t_1()
                    .border_b_1()
                    .border_color(theme.border)
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .h_full()
                            .children(self.render_dock(&self.right_dock))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .flex_1()
                                    .overflow_hidden()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .flex_1()
                                            .child(self.center.clone()),
                                    )
                                    .children(self.render_dock(&self.bottom_dock)),
                            ),
                    ),
            )
    }
}
