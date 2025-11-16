use std::{cell::RefCell, rc::Rc};

use gpui::{
    AnyElement, App, Bounds, Corner, DismissEvent, DispatchPhase, Edges, Element, ElementId,
    Entity, Focusable as _, GlobalElementId, HitboxBehavior, HitboxId, IntoElement, LayoutId,
    Length, ManagedView, MouseDownEvent, ParentElement, Pixels, Point, Style, Window, anchored,
    deferred, div, prelude::FluentBuilder, px, relative, size,
};

use super::PopoverHandle;
use crate::{prelude::*, traits::Toggleable};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PopoverPosition {
    pub anchor: Corner,
    pub attach: Option<Corner>,
    pub offset: Point<Pixels>,
    pub window_margin: Edges<Pixels>,
    pub snap_to_window: bool,
}

impl PopoverPosition {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn anchor(mut self, anchor: Corner) -> Self {
        self.anchor = anchor;
        self
    }

    pub fn attach(mut self, attach: Corner) -> Self {
        self.attach = Some(attach);
        self
    }

    pub fn offset(mut self, offset: Point<Pixels>) -> Self {
        self.offset = offset;
        self
    }

    pub fn window_margin(mut self, margin: Edges<Pixels>) -> Self {
        self.window_margin = margin;
        self
    }

    pub fn snap_to_window(mut self, snap: bool) -> Self {
        self.snap_to_window = snap;
        self
    }

    pub fn resolved_attachment(&self) -> Corner {
        self.attach.unwrap_or_else(|| self.anchor.opposite_corner())
    }

    pub fn compute_position(&self, trigger_bounds: Option<Bounds<Pixels>>) -> Point<Pixels> {
        if let Some(bounds) = trigger_bounds {
            bounds.corner(self.resolved_attachment()) + self.offset
        } else {
            self.offset
        }
    }

    pub fn top() -> Self {
        Self::default()
            .anchor(Corner::BottomLeft)
            .attach(Corner::TopLeft)
    }

    pub fn top_start() -> Self {
        Self::default()
            .anchor(Corner::BottomLeft)
            .attach(Corner::TopLeft)
    }

    pub fn top_end() -> Self {
        Self::default()
            .anchor(Corner::BottomRight)
            .attach(Corner::TopRight)
    }

    pub fn bottom() -> Self {
        Self::default()
            .anchor(Corner::TopLeft)
            .attach(Corner::BottomLeft)
    }

    pub fn bottom_start() -> Self {
        Self::default()
            .anchor(Corner::TopLeft)
            .attach(Corner::BottomLeft)
    }

    pub fn bottom_end() -> Self {
        Self::default()
            .anchor(Corner::TopRight)
            .attach(Corner::BottomRight)
    }

    pub fn left() -> Self {
        Self::default()
            .anchor(Corner::TopRight)
            .attach(Corner::TopLeft)
    }

    pub fn left_start() -> Self {
        Self::default()
            .anchor(Corner::TopRight)
            .attach(Corner::TopLeft)
    }

    pub fn left_end() -> Self {
        Self::default()
            .anchor(Corner::BottomRight)
            .attach(Corner::BottomLeft)
    }

    pub fn right() -> Self {
        Self::default()
            .anchor(Corner::TopLeft)
            .attach(Corner::TopRight)
    }

    pub fn right_start() -> Self {
        Self::default()
            .anchor(Corner::TopLeft)
            .attach(Corner::TopRight)
    }

    pub fn right_end() -> Self {
        Self::default()
            .anchor(Corner::BottomLeft)
            .attach(Corner::BottomRight)
    }
}

impl Default for PopoverPosition {
    fn default() -> Self {
        Self {
            anchor: Corner::BottomRight,
            attach: None,
            window_margin: Edges::all(px(8.0)),
            offset: Point::default(),
            snap_to_window: true,
        }
    }
}

pub trait PopoverTrigger: IntoElement + Clickable + Toggleable + 'static {}

impl<T: IntoElement + Clickable + Toggleable + 'static> PopoverTrigger for T {}

pub struct PopoverElementState<V> {
    view: Rc<RefCell<Option<Entity<V>>>>,
    trigger_bounds: Option<Bounds<Pixels>>,
}

impl<V> Clone for PopoverElementState<V> {
    fn clone(&self) -> Self {
        Self {
            view: Rc::clone(&self.view),
            trigger_bounds: self.trigger_bounds,
        }
    }
}

impl<V> Default for PopoverElementState<V> {
    fn default() -> Self {
        Self {
            view: Rc::default(),
            trigger_bounds: None,
        }
    }
}

pub struct PopoverFrameState<V: ManagedView> {
    trigger_layout_id: Option<LayoutId>,
    trigger_element: Option<AnyElement>,
    view_element: Option<AnyElement>,
    view_handle: Rc<RefCell<Option<Entity<V>>>>,
}

pub struct Popover<V: ManagedView> {
    id: ElementId,
    trigger_builder: Option<
        Box<
            dyn FnOnce(
                    Rc<RefCell<Option<Entity<V>>>>,
                    Option<Rc<dyn Fn(&mut Window, &mut App) -> Option<Entity<V>> + 'static>>,
                ) -> AnyElement
                + 'static,
        >,
    >,
    view_builder: Option<Rc<dyn Fn(&mut Window, &mut App) -> Option<Entity<V>> + 'static>>,
    handle: Option<PopoverHandle<V>>,
    position: PopoverPosition,
    layout: Layout,
}

impl<V: ManagedView> Popover<V> {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            trigger_builder: None,
            view_builder: None,
            position: PopoverPosition::default(),
            handle: None,
            layout: Layout::Standalone,
        }
    }

    pub fn content(
        mut self,
        builder: impl Fn(&mut Window, &mut App) -> Option<Entity<V>> + 'static,
    ) -> Self {
        self.view_builder = Some(Rc::new(builder));
        self
    }

    pub fn trigger<T: PopoverTrigger>(mut self, trigger: T) -> Self {
        self.trigger_builder = Some(Box::new(move |view, builder| {
            let is_open = view.borrow().is_some();

            trigger
                .toggle_state(is_open)
                .when_some(builder, |this, builder| {
                    this.on_click(move |_event, window, cx| {
                        let Some(new_view) = (builder)(window, cx) else {
                            return;
                        };

                        let view_clone = view.clone();
                        let previous_focus = window.focused(cx);

                        window
                            .subscribe(
                                &new_view,
                                cx,
                                move |entity, _: &DismissEvent, window, cx| {
                                    if entity.focus_handle(cx).contains_focused(window, cx)
                                        && let Some(previous_focus) = previous_focus.as_ref()
                                    {
                                        window.focus(previous_focus);
                                    }

                                    *view_clone.borrow_mut() = None;

                                    window.refresh();
                                },
                            )
                            .detach();

                        window.focus(&new_view.focus_handle(cx));
                        *view.borrow_mut() = Some(new_view);

                        window.refresh();
                    })
                })
                .into_any_element()
        }));

        self
    }

    pub fn position(mut self, position: PopoverPosition) -> Self {
        self.position = position;
        self
    }

    pub fn with_handle(mut self, handle: PopoverHandle<V>) -> Self {
        self.handle = Some(handle);
        self
    }
}

impl<V: ManagedView> Layoutable for Popover<V> {
    fn layout_variant(mut self, variant: Layout) -> Self {
        self.layout = variant;
        self
    }
}

impl<V: ManagedView> Element for Popover<V> {
    type RequestLayoutState = PopoverFrameState<V>;
    type PrepaintState = Option<HitboxId>;

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        global_id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        window.with_element_state(
            global_id.unwrap(),
            |element_state: Option<PopoverElementState<V>>, window| {
                let element_state = element_state.unwrap_or_default();
                let mut view_layout_id = None;

                let view_element = element_state.view.borrow_mut().as_mut().map(|view| {
                    let position = self.position.compute_position(element_state.trigger_bounds);
                    let mut anchored_element = anchored()
                        .anchor(self.position.anchor)
                        .offset(self.position.offset)
                        .position(position);

                    if self.position.snap_to_window {
                        anchored_element = anchored_element
                            .snap_to_window_with_margin(self.position.window_margin);
                    }

                    let mut element =
                        deferred(anchored_element.child(div().occlude().child(view.clone())))
                            .with_priority(1)
                            .into_any();

                    view_layout_id = Some(element.request_layout(window, cx));
                    element
                });

                let mut trigger_element = self.trigger_builder.take().map(|builder| {
                    (builder)(element_state.view.clone(), self.view_builder.clone())
                });

                if let Some(handle) = self.handle.take()
                    && let Some(view_builder) = self.view_builder.clone()
                {
                    handle.initialize(view_builder, element_state.view.clone());
                }

                let trigger_layout_id = trigger_element
                    .as_mut()
                    .map(|element| element.request_layout(window, cx));

                let mut style = Style::default();
                if self.layout.is_full_width() {
                    style.size = size(relative(1.).into(), Length::Auto);
                }

                let layout_id = window.request_layout(
                    style,
                    view_layout_id.into_iter().chain(trigger_layout_id),
                    cx,
                );

                (
                    (
                        layout_id,
                        PopoverFrameState {
                            trigger_element,
                            trigger_layout_id,
                            view_element,
                            view_handle: element_state.view.clone(),
                        },
                    ),
                    element_state,
                )
            },
        )
    }

    fn prepaint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        _bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<HitboxId> {
        if let Some(trigger) = request_layout.trigger_element.as_mut() {
            trigger.prepaint(window, cx);
        }

        if let Some(view) = request_layout.view_element.as_mut() {
            view.prepaint(window, cx);
        }

        request_layout.trigger_layout_id.map(|layout_id| {
            let bounds = window.layout_bounds(layout_id);
            window.with_element_state(global_id.unwrap(), |element_state, _cx| {
                let mut element_state: PopoverElementState<V> = element_state.unwrap();
                element_state.trigger_bounds = Some(bounds);
                ((), element_state)
            });

            window.insert_hitbox(bounds, HitboxBehavior::Normal).id
        })
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        _bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        trigger_hitbox: &mut Option<HitboxId>,
        window: &mut Window,
        cx: &mut App,
    ) {
        if let Some(mut trigger) = request_layout.trigger_element.take() {
            trigger.paint(window, cx);
        }

        if let Some(mut view) = request_layout.view_element.take() {
            view.paint(window, cx);

            if let Some(trigger_hitbox) = *trigger_hitbox {
                let view_handle = request_layout.view_handle.clone();

                window.on_mouse_event(move |_: &MouseDownEvent, phase, window, cx| {
                    if phase == DispatchPhase::Bubble && trigger_hitbox.is_hovered(window) {
                        if let Some(view) = view_handle.borrow().as_ref() {
                            view.update(cx, |_, cx| {
                                cx.emit(DismissEvent);
                            });
                        }
                        cx.stop_propagation();
                    }
                })
            }
        }
    }
}

impl<V: ManagedView> IntoElement for Popover<V> {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}
