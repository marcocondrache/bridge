use std::sync::Arc;

use gpui::{
    Action, AnyView, App, AppContext, Axis, Context, Entity, EntityId, FocusHandle, Focusable,
    ParentElement, Pixels, Render, StyleRefinement, Styled, WeakEntity, Window, div,
    prelude::FluentBuilder,
};

use crate::{Workspace, pane::Pane};

pub trait Panel: Focusable + Render + Sized {
    fn persistent_name() -> &'static str;
    fn panel_key() -> &'static str;
    fn position(&self, window: &Window, cx: &App) -> DockPlacement;
    fn position_is_valid(&self, position: DockPlacement) -> bool;
    fn set_position(
        &mut self,
        position: DockPlacement,
        window: &mut Window,
        cx: &mut Context<Self>,
    );
    fn size(&self, window: &Window, cx: &App) -> Pixels;
    fn set_size(&mut self, size: Option<Pixels>, window: &mut Window, cx: &mut Context<Self>);
    fn toggle_action(&self) -> Box<dyn Action>;
    fn icon_label(&self, _window: &Window, _: &App) -> Option<String> {
        None
    }
    fn is_zoomed(&self, _window: &Window, _cx: &App) -> bool {
        false
    }
    fn starts_open(&self, _window: &Window, _cx: &App) -> bool {
        false
    }
    fn set_zoomed(&mut self, _zoomed: bool, _window: &mut Window, _cx: &mut Context<Self>) {}
    fn set_active(&mut self, _active: bool, _window: &mut Window, _cx: &mut Context<Self>) {}
    fn pane(&self) -> Option<Entity<Pane>> {
        None
    }
    fn activation_priority(&self) -> u32;
    fn enabled(&self, _cx: &App) -> bool {
        true
    }
}

pub trait PanelHandle: Send + Sync {
    fn panel_id(&self) -> EntityId;
    fn persistent_name(&self) -> &'static str;
    fn panel_key(&self) -> &'static str;
    fn position(&self, window: &Window, cx: &App) -> DockPlacement;
    fn position_is_valid(&self, position: DockPlacement, cx: &App) -> bool;
    fn set_position(&self, position: DockPlacement, window: &mut Window, cx: &mut App);
    fn is_zoomed(&self, window: &Window, cx: &App) -> bool;
    fn set_zoomed(&self, zoomed: bool, window: &mut Window, cx: &mut App);
    fn set_active(&self, active: bool, window: &mut Window, cx: &mut App);
    fn pane(&self, cx: &App) -> Option<Entity<Pane>>;
    fn size(&self, window: &Window, cx: &App) -> Pixels;
    fn set_size(&self, size: Option<Pixels>, window: &mut Window, cx: &mut App);
    fn toggle_action(&self, window: &Window, cx: &App) -> Box<dyn Action>;
    fn icon_label(&self, window: &Window, cx: &App) -> Option<String>;
    fn panel_focus_handle(&self, cx: &App) -> FocusHandle;
    fn to_any(&self) -> AnyView;
    fn activation_priority(&self, cx: &App) -> u32;
    fn enabled(&self, cx: &App) -> bool;
    fn move_to_next_position(&self, window: &mut Window, cx: &mut App) {
        let current_position = self.position(window, cx);
        let next_position = [DockPlacement::Left, DockPlacement::Bottom]
            .into_iter()
            .filter(|position| self.position_is_valid(*position, cx))
            .skip_while(|valid_position| *valid_position != current_position)
            .nth(1)
            .unwrap_or(DockPlacement::Left);

        self.set_position(next_position, window, cx);
    }
}

impl<T> PanelHandle for Entity<T>
where
    T: Panel,
{
    fn panel_id(&self) -> EntityId {
        Entity::entity_id(self)
    }

    fn persistent_name(&self) -> &'static str {
        T::persistent_name()
    }

    fn panel_key(&self) -> &'static str {
        T::panel_key()
    }

    fn position(&self, window: &Window, cx: &App) -> DockPlacement {
        self.read(cx).position(window, cx)
    }

    fn position_is_valid(&self, position: DockPlacement, cx: &App) -> bool {
        self.read(cx).position_is_valid(position)
    }

    fn set_position(&self, position: DockPlacement, window: &mut Window, cx: &mut App) {
        self.update(cx, |this, cx| this.set_position(position, window, cx))
    }

    fn is_zoomed(&self, window: &Window, cx: &App) -> bool {
        self.read(cx).is_zoomed(window, cx)
    }

    fn set_zoomed(&self, zoomed: bool, window: &mut Window, cx: &mut App) {
        self.update(cx, |this, cx| this.set_zoomed(zoomed, window, cx))
    }

    fn set_active(&self, active: bool, window: &mut Window, cx: &mut App) {
        self.update(cx, |this, cx| this.set_active(active, window, cx))
    }

    fn pane(&self, cx: &App) -> Option<Entity<Pane>> {
        self.read(cx).pane()
    }

    fn size(&self, window: &Window, cx: &App) -> Pixels {
        self.read(cx).size(window, cx)
    }

    fn set_size(&self, size: Option<Pixels>, window: &mut Window, cx: &mut App) {
        self.update(cx, |this, cx| this.set_size(size, window, cx))
    }

    fn toggle_action(&self, _: &Window, cx: &App) -> Box<dyn Action> {
        self.read(cx).toggle_action()
    }

    fn icon_label(&self, window: &Window, cx: &App) -> Option<String> {
        self.read(cx).icon_label(window, cx)
    }

    fn to_any(&self) -> AnyView {
        self.clone().into()
    }

    fn panel_focus_handle(&self, cx: &App) -> FocusHandle {
        self.read(cx).focus_handle(cx)
    }

    fn activation_priority(&self, cx: &App) -> u32 {
        self.read(cx).activation_priority()
    }

    fn enabled(&self, cx: &App) -> bool {
        self.read(cx).enabled(cx)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DockPlacement {
    Left,
    Bottom,
}

impl DockPlacement {
    fn label(&self) -> &'static str {
        match self {
            Self::Left => "Left",
            Self::Bottom => "Bottom",
        }
    }

    pub fn axis(&self) -> Axis {
        match self {
            Self::Left => Axis::Horizontal,
            Self::Bottom => Axis::Vertical,
        }
    }
}

pub struct Dock {
    placement: DockPlacement,
    panel_entries: Vec<Arc<dyn PanelHandle>>,
    is_open: bool,
    active_panel_index: Option<usize>,
}

impl Dock {
    pub fn new(
        placement: DockPlacement,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        cx.new(|cx| Self {
            placement,
            panel_entries: Vec::new(),
            is_open: false,
            active_panel_index: None,
        })
    }

    pub fn placement(&self) -> DockPlacement {
        self.placement
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn active_panel_index(&self) -> Option<usize> {
        self.active_panel_index
    }

    pub fn panels_len(&self) -> usize {
        self.panel_entries.len()
    }

    pub fn activate_panel(&mut self, panel_ix: usize, window: &mut Window, cx: &mut Context<Self>) {
        if Some(panel_ix) != self.active_panel_index {
            if let Some(active_panel) = self.active_panel() {
                active_panel.set_active(false, window, cx);
            }

            self.active_panel_index = Some(panel_ix);
            if let Some(active_panel) = self.active_panel() {
                active_panel.set_active(true, window, cx);
            }

            cx.notify();
        }
    }

    pub fn set_open(&mut self, open: bool, window: &mut Window, cx: &mut Context<Self>) {
        if open != self.is_open {
            self.is_open = open;

            if let Some(active_panel) = self.active_panel() {
                active_panel.set_active(open, window, cx);
            }

            cx.notify();
        }
    }

    pub fn add_panel<T: Panel>(
        &mut self,
        panel: Entity<T>,
        // TODO: subscribe to changes
        _workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let search = self
            .panel_entries
            .binary_search_by_key(&panel.read(cx).activation_priority(), |panel| {
                panel.activation_priority(cx)
            });

        let index = match search {
            Ok(index) => index,
            Err(index) => index,
        };

        if let Some(active_index) = self.active_panel_index.as_mut()
            && *active_index >= index
        {
            *active_index += 1;
        }

        self.panel_entries.insert(index, Arc::new(panel.clone()));

        if panel.read(cx).starts_open(window, cx) {
            self.activate_panel(index, window, cx);
            self.set_open(true, window, cx);
        }

        cx.notify();
    }

    pub fn remove_panel<T: Panel>(
        &mut self,
        panel: &Entity<T>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(index) = self
            .panel_entries
            .iter()
            .position(|p| p.panel_id() == panel.entity_id())
        {
            if let Some(active_index) = self.active_panel_index.as_mut() {
                match index.cmp(active_index) {
                    std::cmp::Ordering::Less => {
                        *active_index -= 1;
                    }
                    std::cmp::Ordering::Equal => {
                        self.active_panel_index = None;
                        self.set_open(false, window, cx);
                    }
                    _ => {}
                }
            }

            self.panel_entries.remove(index);

            cx.notify();
        }
    }

    fn active_panel(&self) -> Option<&Arc<dyn PanelHandle>> {
        self.active_panel_index
            .and_then(|index| self.panel_entries.get(index))
    }

    fn visible_panel(&self) -> Option<&Arc<dyn PanelHandle>> {
        if self.is_open {
            self.active_panel()
        } else {
            None
        }
    }
}

impl Render for Dock {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        if let Some(panel) = self.visible_panel() {
            let size = panel.size(window, cx);

            div()
                .flex()
                .overflow_hidden()
                .map(|this| match self.placement().axis() {
                    Axis::Vertical => this.w(size).h_full().flex_row(),
                    Axis::Horizontal => this.h(size).w_full().flex_col(),
                })
                .map(|this| match self.placement() {
                    DockPlacement::Left => this.border_r_1(),
                    DockPlacement::Bottom => this.border_t_1(),
                })
                .child(
                    div()
                        .map(|this| match self.placement().axis() {
                            Axis::Horizontal => this.min_w(size).h_full(),
                            Axis::Vertical => this.min_h(size).w_full(),
                        })
                        .child(
                            panel
                                .to_any()
                                .cached(StyleRefinement::default().flex().flex_col().size_full()),
                        ),
                )
        } else {
            div()
        }
    }
}
