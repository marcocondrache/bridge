use std::collections::HashMap;

use gpui::{
    App, AppContext, Context, ElementId, Entity, EventEmitter, FocusHandle, Focusable,
    InteractiveElement, IntoElement, ParentElement, Render, StatefulInteractiveElement, Styled,
    Window, actions, div, prelude::FluentBuilder, px,
};
use gpui_component::ActiveTheme;

use super::table::Table;
use crate::components::input::Input;

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
    pub key: String,
    pub value: String,
}

impl KeyValueRow {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            enabled: true,
            key: String::new(),
            value: String::new(),
        }
    }

    pub fn with_key(mut self, key: impl Into<String>) -> Self {
        self.key = key.into();
        self
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
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

    fn set_content(&mut self, cell_type: CellType, content: String) {
        match cell_type {
            CellType::Key => self.key = content,
            CellType::Value => self.value = content,
            CellType::Enabled => {}
        }
    }
}

pub struct TableKV {
    rows: Vec<KeyValueRow>,
    active_cell: Option<(usize, CellType)>,
    active_input: Option<Entity<Input>>,
    focus_handles: HashMap<(usize, CellType), FocusHandle>,
    next_row_id: usize,
}

impl TableKV {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            rows: vec![KeyValueRow::new("row_0")],
            active_cell: None,
            active_input: None,
            focus_handles: HashMap::new(),
            next_row_id: 1,
        }
    }

    pub fn with_rows(mut self, rows: Vec<KeyValueRow>) -> Self {
        self.rows = rows;
        self
    }

    pub fn rows(&self) -> &[KeyValueRow] {
        &self.rows
    }

    fn ensure_focus_handles(&mut self, cx: &mut Context<Self>) {
        for (row_idx, row) in self.rows.iter().enumerate() {
            for cell_type in [CellType::Key, CellType::Value] {
                let key = (row_idx, cell_type);
                if !self.focus_handles.contains_key(&key) {
                    self.focus_handles.insert(key, cx.focus_handle());
                }
            }
        }
    }

    fn focus_cell(
        &mut self,
        row_idx: usize,
        cell_type: CellType,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if cell_type == CellType::Enabled {
            return;
        }

        // Save previous cell content if exists
        if let Some((prev_row, prev_cell)) = self.active_cell {
            if let Some(input) = &self.active_input {
                let content = input.read(cx).get_content();
                if prev_row < self.rows.len() {
                    self.rows[prev_row].set_content(prev_cell, content);
                }
            }
        }

        // Get current cell content
        if row_idx >= self.rows.len() {
            return;
        }

        let content = self.rows[row_idx].get_content(cell_type).to_string();

        // Create new Input with current cell content
        let input = cx.new(|cx| Input::new(cx).content(&content));

        self.active_input = Some(input.clone());
        self.active_cell = Some((row_idx, cell_type));

        // Focus the input
        let focus_handle = input.read(cx).focus_handle(cx).clone();
        window.focus(&focus_handle);

        cx.notify();
    }

    fn blur_cell(&mut self, cx: &mut Context<Self>) {
        // Save content back to cell
        if let Some((row_idx, cell_type)) = self.active_cell {
            if let Some(input) = &self.active_input {
                if row_idx < self.rows.len() {
                    let content = input.read(cx).get_content();
                    self.rows[row_idx].set_content(cell_type, content);
                }
            }
        }

        // Destroy the input
        self.active_input = None;
        self.active_cell = None;
        cx.notify();
    }

    fn add_row(&mut self, _: &AddRow, _: &mut Window, cx: &mut Context<Self>) {
        let new_row = KeyValueRow::new(("row", self.next_row_id));
        self.next_row_id += 1;
        self.rows.push(new_row);
        cx.notify();
    }

    fn delete_row(&mut self, row_idx: usize, cx: &mut Context<Self>) {
        if self.rows.len() <= 1 {
            return;
        }

        // If deleting the active cell, blur it first
        if let Some((active_row, _)) = self.active_cell {
            if active_row == row_idx {
                self.blur_cell(cx);
            }
        }

        self.rows.remove(row_idx);

        // Clean up focus handles for this row
        self.focus_handles.retain(|(idx, _), _| *idx != row_idx);

        // Adjust indices for rows after the deleted one
        let mut new_handles = HashMap::new();
        for ((idx, cell_type), handle) in self.focus_handles.drain() {
            if idx > row_idx {
                new_handles.insert((idx - 1, cell_type), handle);
            } else {
                new_handles.insert((idx, cell_type), handle);
            }
        }
        self.focus_handles = new_handles;

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
                    // Render the actual Input component
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
                    // Render plain text - clickable to focus
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
                        .child(if content.is_empty() {
                            div()
                                .text_color(theme.muted_foreground)
                                .child(match cell_type {
                                    CellType::Key => "Key",
                                    CellType::Value => "Value",
                                    CellType::Enabled => "",
                                })
                        } else {
                            div().child(content)
                        })
                }
            }
        }
    }

    fn render_delete_button(&self, row_idx: usize, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        div().id("").flex().items_center().justify_center().child(
            div()
                .id("")
                .w(px(20.))
                .h(px(20.))
                .flex()
                .items_center()
                .justify_center()
                .border_1()
                .border_color(theme.border)
                .rounded(px(3.))
                .cursor_pointer()
                .hover(|this| this.bg(theme.danger))
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.delete_row(row_idx, cx);
                }))
                .child("×"),
        )
    }
}

impl Focusable for TableKV {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handles
            .values()
            .next()
            .cloned()
            .unwrap_or_else(|| panic!("TableKV should have at least one focus handle"))
    }
}

impl Render for TableKV {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.ensure_focus_handles(cx);

        let rows = self
            .rows
            .clone()
            .iter()
            .enumerate()
            .map(|(idx, _row)| {
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

        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                Table::<4>::new()
                    .header(["", "Key", "Value", ""])
                    .rows(rows)
                    .width(gpui::relative(1.)),
            )
            .child(
                div().flex().justify_start().child(
                    div()
                        .id("")
                        .px_2()
                        .py_1()
                        .border_1()
                        .rounded(px(4.))
                        .cursor_pointer()
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.add_row(&AddRow, window, cx);
                        }))
                        .child("+ Add Row"),
                ),
            )
    }
}

impl EventEmitter<()> for TableKV {}
