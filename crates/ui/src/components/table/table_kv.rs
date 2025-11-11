use gpui::{
    App, AppContext, Context, ElementId, Entity, FocusHandle, Focusable, InteractiveElement,
    IntoElement, ParentElement, Render, SharedString, StatefulInteractiveElement, Styled, Window,
    actions, div, prelude::FluentBuilder, px,
};
use gpui_component::ActiveTheme;

use super::table::Table;
use crate::{
    components::{button::Button, input::Input, label::Label},
    traits::clickable::Clickable,
};

actions!(
    table_kv,
    [AddRow, DeleteRow, MoveToNextCell, MoveToPrevCell]
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CellType {
    Enabled,
    Key,
    Value,
}

#[derive(Debug, Clone)]
pub struct KeyValueRow {
    pub id: ElementId,
    pub enabled: bool,
    pub key: SharedString,
    pub value: SharedString,
}

impl KeyValueRow {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            enabled: true,
            key: SharedString::default(),
            value: SharedString::default(),
        }
    }

    pub fn with_key(mut self, key: impl Into<SharedString>) -> Self {
        self.key = key.into();
        self
    }

    pub fn with_value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = value.into();
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    fn get_content(&self, cell_type: CellType) -> &str {
        match cell_type {
            CellType::Key => &self.key,
            CellType::Value => &self.value,
            CellType::Enabled => "",
        }
    }

    fn set_content(&mut self, cell_type: CellType, content: impl Into<SharedString>) {
        match cell_type {
            CellType::Key => self.key = content.into(),
            CellType::Value => self.value = content.into(),
            CellType::Enabled => {}
        }
    }
}

pub struct TableKV {
    rows: Vec<KeyValueRow>,
    active_cell: Option<(usize, CellType)>,
    active_input: Option<Entity<Input>>,
    focus_handle: FocusHandle,
}

impl TableKV {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            rows: vec![],
            active_cell: None,
            active_input: None,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn with_rows(mut self, rows: Vec<KeyValueRow>) -> Self {
        self.rows = rows;
        self
    }

    pub fn rows(&self) -> &[KeyValueRow] {
        &self.rows
    }

    fn focus_cell(
        &mut self,
        index: usize,
        cell_type: CellType,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some((prev_row, prev_cell)) = self.active_cell {
            if let Some(input) = &self.active_input {
                let content = input.read(cx).get_content();
                if prev_row < self.rows.len() {
                    self.rows[prev_row].set_content(prev_cell, content);
                }
            }
        }

        let content = self.rows[index].get_content(cell_type).to_string();
        let input = cx.new(|cx| Input::new(cx).content(&content));

        self.active_input = Some(input.clone());
        self.active_cell = Some((index, cell_type));

        window.focus(&input.read(cx).focus_handle(cx));

        cx.notify();
    }

    fn blur_cell(&mut self, cx: &mut Context<Self>) {
        if let Some((row_idx, cell_type)) = self.active_cell {
            if let Some(input) = &self.active_input {
                if row_idx < self.rows.len() {
                    let content = input.read(cx).get_content();
                    self.rows[row_idx].set_content(cell_type, content);
                }
            }
        }

        self.active_input = None;
        self.active_cell = None;
        cx.notify();
    }

    fn add_row(&mut self, _: &AddRow, _: &mut Window, cx: &mut Context<Self>) {
        let id = self.rows.len();
        self.rows.push(KeyValueRow::new(("row", id)));
        cx.notify();
    }

    fn delete_row(&mut self, row_idx: usize, cx: &mut Context<Self>) {
        if self.rows.len() <= 1 {
            return;
        }

        if let Some((active_row, _)) = self.active_cell {
            if active_row == row_idx {
                self.blur_cell(cx);
            }
        }

        self.rows.remove(row_idx);

        cx.notify();
    }

    fn toggle_enabled(&mut self, row_idx: usize, cx: &mut Context<Self>) {
        if row_idx < self.rows.len() {
            self.rows[row_idx].enabled = !self.rows[row_idx].enabled;
            cx.notify();
        }
    }

    fn render_cell(
        &mut self,
        row_idx: usize,
        cell_type: CellType,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let is_active = self.active_cell == Some((row_idx, cell_type));

        match cell_type {
            CellType::Enabled => {
                let enabled = self.rows[row_idx].enabled;
                div().id("").flex().items_center().justify_center().child(
                    div()
                        .id("")
                        .w(px(16.))
                        .h(px(16.))
                        .border_1()
                        .border_color(theme.border)
                        .rounded(px(3.))
                        .when(enabled, |this| this.bg(theme.primary))
                        .cursor_pointer()
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.toggle_enabled(row_idx, cx);
                        })),
                )
            }
            CellType::Key | CellType::Value => {
                if is_active {
                    div()
                        .id("")
                        .w_full()
                        .h_full()
                        .border_1()
                        .border_color(theme.primary)
                        .rounded(px(2.))
                        .px_1()
                        .when_some(self.active_input.clone(), |this, input| this.child(input))
                } else {
                    let content = self.rows[row_idx].get_content(cell_type).to_string();
                    let is_enabled = self.rows[row_idx].enabled;

                    div()
                        .id("")
                        .w_full()
                        .h_full()
                        .border_1()
                        .border_color(theme.border)
                        .rounded(px(2.))
                        .px_1()
                        .cursor_text()
                        .when(!is_enabled, |this| this.text_color(theme.muted_foreground))
                        .on_click(cx.listener(move |this, _, window, cx| {
                            this.focus_cell(row_idx, cell_type, window, cx);
                        }))
                        .child(Label::new(content))
                }
            }
        }
    }

    fn render_delete_button(&self, row_idx: usize, cx: &mut Context<Self>) -> impl IntoElement {
        Button::new(0)
            .label("x")
            .on_click(cx.listener(move |this, _, _, cx| {
                this.delete_row(row_idx, cx);
            }))
    }
}

impl Focusable for TableKV {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.active_input.as_ref().map_or_else(
            || self.focus_handle.clone(),
            |input| input.read(cx).focus_handle(cx),
        )
    }
}

impl Render for TableKV {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let rows = self
            .rows
            .clone()
            .iter()
            .enumerate()
            .map(|(idx, _)| {
                [
                    self.render_cell(idx, CellType::Enabled, cx)
                        .into_any_element(),
                    self.render_cell(idx, CellType::Key, cx).into_any_element(),
                    self.render_cell(idx, CellType::Value, cx)
                        .into_any_element(),
                    self.render_delete_button(idx, cx).into_any_element(),
                ]
            })
            .collect::<Vec<_>>();

        Table::<4>::new()
            .header(["", "Key", "Value", ""])
            .rows(rows)
    }
}
