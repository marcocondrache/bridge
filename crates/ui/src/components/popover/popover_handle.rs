use std::{cell::RefCell, rc::Rc};

use gpui::{App, DismissEvent, Entity, Focusable, ManagedView, Window};

pub struct PopoverHandle<V>(Rc<RefCell<Option<PopoverHandleState<V>>>>);

impl<V> Clone for PopoverHandle<V> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<V> Default for PopoverHandle<V> {
    fn default() -> Self {
        Self(Rc::default())
    }
}

struct PopoverHandleState<V> {
    view_builder: Rc<dyn Fn(&mut Window, &mut App) -> Option<Entity<V>>>,
    view: Rc<RefCell<Option<Entity<V>>>>,
}

impl<V: ManagedView> PopoverHandle<V> {
    pub fn show(&self, window: &mut Window, cx: &mut App) {
        if let Some(state) = self.0.borrow().as_ref() {
            show_popover(&state.view_builder, &state.view, window, cx);
        }
    }

    pub fn hide(&self, cx: &mut App) {
        if let Some(state) = self.0.borrow().as_ref()
            && let Some(view) = state.view.borrow().as_ref()
        {
            view.update(cx, |_, cx| cx.emit(DismissEvent));
        }
    }

    pub fn toggle(&self, window: &mut Window, cx: &mut App) {
        if let Some(state) = self.0.borrow().as_ref() {
            if state.view.borrow().is_some() {
                self.hide(cx);
            } else {
                self.show(window, cx);
            }
        }
    }

    pub fn is_open(&self) -> bool {
        self.0
            .borrow()
            .as_ref()
            .is_some_and(|state| state.view.borrow().as_ref().is_some())
    }

    pub fn is_focused(&self, window: &Window, cx: &App) -> bool {
        self.0.borrow().as_ref().is_some_and(|state| {
            state
                .view
                .borrow()
                .as_ref()
                .is_some_and(|entity| entity.focus_handle(cx).is_focused(window))
        })
    }

    pub fn refresh(
        &self,
        window: &mut Window,
        cx: &mut App,
        new_builder: Rc<dyn Fn(&mut Window, &mut App) -> Option<Entity<V>>>,
    ) {
        let should_show = if let Some(state) = self.0.borrow_mut().as_mut() {
            state.view_builder = new_builder;
            state.view.borrow().is_some()
        } else {
            false
        };

        if should_show {
            self.show(window, cx);
        }
    }

    pub(super) fn initialize(
        &self,
        view_builder: Rc<dyn Fn(&mut Window, &mut App) -> Option<Entity<V>>>,
        view: Rc<RefCell<Option<Entity<V>>>>,
    ) {
        *self.0.borrow_mut() = Some(PopoverHandleState { view_builder, view });
    }
}

fn show_popover<V: ManagedView>(
    builder: &Rc<dyn Fn(&mut Window, &mut App) -> Option<Entity<V>>>,
    view: &Rc<RefCell<Option<Entity<V>>>>,
    window: &mut Window,
    cx: &mut App,
) {
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
}
