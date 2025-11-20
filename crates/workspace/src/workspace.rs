pub mod area;
pub mod dock;
pub mod navigation_bar;

use std::sync::{Arc, Weak};

use anyhow::Ok;
use gpui::{
    Action, AnyView, App, AppContext, Bounds, Context, Div, DragMoveEvent, Entity, Global,
    InteractiveElement, IntoElement, ParentElement, Pixels, Render, Styled, Subscription, Task,
    Window, WindowHandle, WindowOptions, actions, canvas, div,
};
use gpui_component::Root;
use ui::{components::resize_handle::ResizeHandle, utils::placement::Placement};
use uuid::Uuid;

use crate::{
    area::{Area, ItemHandle},
    dock::{Dock, DockButtons, Panel, PanelHandle},
    navigation_bar::NavigationBar,
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
    left_dock: Entity<Dock>,
    bottom_dock: Entity<Dock>,
    center: Entity<Area>,
    titlebar_item: Option<AnyView>,
    navigation_bar: Entity<NavigationBar>,
    bounds: Bounds<Pixels>,
    actions: Vec<Box<dyn Fn(Div, &Workspace, &mut Window, &mut Context<Self>) -> Div>>,
    _subscriptions: Vec<Subscription>,
}

impl Workspace {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let left_dock = Dock::new(Placement::Left, cx);
        let bottom_dock = Dock::new(Placement::Bottom, cx);
        let center = Area::new(window, cx);
        let left_dock_buttons = cx.new(|cx| DockButtons::new(left_dock.clone(), cx));
        let navigation_bar = cx.new(|cx| {
            let mut bar = NavigationBar::new(window, cx);
            bar.add_item(left_dock_buttons, window, cx);

            bar
        });

        let subscriptions = vec![];

        Self {
            left_dock,
            bottom_dock,
            center,
            actions: Default::default(),
            bounds: Default::default(),
            titlebar_item: None,
            navigation_bar,
            _subscriptions: subscriptions,
        }
    }

    pub fn add_panel<T: Panel>(
        &mut self,
        panel: Entity<T>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> usize {
        let placement = panel.placement(window, cx);
        let dock = self.dock_at_placement(placement);

        dock.update(cx, |dock, cx| dock.add_panel(panel, cx))
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

    pub fn activate_panel(&mut self, index: usize, placement: Placement, cx: &mut Context<Self>) {
        let dock = self.dock_at_placement(placement);

        dock.update(cx, |dock, cx| {
            dock.display_panel(Some(index), cx);
        })
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

    pub fn resize_dock(
        &mut self,
        placement: Placement,
        new_size: Pixels,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let dock = self.dock_at_placement(placement);
        let size = match placement {
            Placement::Top => {
                let max_size = self.bounds.bottom()
                    - ResizeHandle::<DraggedDock>::HANDLE_SIZE
                    - self.bounds.top();
                let size = new_size.min(max_size);

                let opposite_dock = self.dock_at_placement(placement.opposite());
                let opposite_size = opposite_dock.read(cx).size().unwrap_or(Pixels::ZERO);

                let available_height = self.bounds.bottom() - self.bounds.top() - opposite_size;

                size.min(available_height)
            }
            Placement::Right => {
                let max_size = self.bounds.right() - ResizeHandle::<DraggedDock>::HANDLE_SIZE;
                let size = new_size.min(max_size);

                let opposite_dock = self.dock_at_placement(placement.opposite());
                let opposite_size = opposite_dock.read(cx).size().unwrap_or(Pixels::ZERO);

                let available_width = self.bounds.right() - opposite_size;

                size.min(available_width)
            }
            Placement::Bottom => new_size.min(
                self.bounds.bottom() - ResizeHandle::<DraggedDock>::HANDLE_SIZE - self.bounds.top(),
            ),
            Placement::Left => {
                new_size.min(self.bounds.right() - ResizeHandle::<DraggedDock>::HANDLE_SIZE)
            }
        };

        dock.update(cx, |dock, cx| {
            dock.resize(Some(size), cx);
        });
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

                    cx.new(|cx| Root::new(workspace, window, cx))
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
            Placement::Left => &self.left_dock,
            Placement::Bottom => &self.bottom_dock,
            _ => &self.left_dock,
        }
    }
}

pub fn open_new(app_state: Arc<AppState>, cx: &mut App) {
    let task = Workspace::spawn(app_state, cx);

    cx.spawn(async move |_| {
        let _ = task.await;
    })
    .detach();
}

#[derive(Clone)]
pub(crate) struct DraggedDock(Placement);

impl Render for DraggedDock {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        gpui::Empty
    }
}

impl Render for Workspace {
    fn render(
        &mut self,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        let modal_layer = Root::render_dialog_layer(window, cx);
        let notification_layer = Root::render_notification_layer(window, cx);

        self.actions(div(), window, cx)
            .id("root")
            .relative()
            .size_full()
            .flex()
            .flex_col()
            .child({
                let this = cx.entity();
                canvas(
                    move |bounds, _window, cx| {
                        this.update(cx, |this, _cx| {
                            this.bounds = bounds;
                        })
                    },
                    |_, _, _, _| {},
                )
                .absolute()
                .size_full()
            })
            .on_drag_move(cx.listener(
                move |workspace, e: &DragMoveEvent<DraggedDock>, window, cx| {
                    let placement = e.drag(cx).0;
                    let new_size = match placement {
                        Placement::Top => e.event.position.y - workspace.bounds.top(),
                        Placement::Bottom => workspace.bounds.bottom() - e.event.position.y,
                        Placement::Right => workspace.bounds.right() - e.event.position.x,
                        Placement::Left => e.event.position.x - workspace.bounds.left(),
                    };

                    workspace.resize_dock(placement, new_size, window, cx)
                },
            ))
            .children(self.titlebar_item.clone())
            .child(
                div()
                    .flex()
                    .flex_row()
                    .h_full()
                    .child(self.navigation_bar.clone())
                    .child(div().flex().flex_none().child(self.left_dock.clone()))
                    .child(
                        div()
                            .flex()
                            .flex_1()
                            .flex_col()
                            .overflow_hidden()
                            .child(self.center.clone())
                            .child(self.bottom_dock.clone()),
                    ),
            )
            .children(modal_layer)
            .children(notification_layer)
    }
}
