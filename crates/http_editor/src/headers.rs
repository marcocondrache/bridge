use gpui::{App, AppContext, Context, Entity, ParentElement, Window, div, prelude::FluentBuilder};
use gpui_component::{
    input::{Input, InputState},
    table::{Column, ColumnFixed, TableDelegate, TableState},
};
use indexmap::IndexSet;

pub type HeadersTable = TableState<HeadersTableDelegate>;

pub fn headers_table(window: &mut Window, cx: &mut Context<HeadersTable>) -> HeadersTable {
    TableState::new(HeadersTableDelegate::new(), window, cx).row_selectable(false)
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Row {
    name: Entity<InputState>,
    value: Entity<InputState>,
}

impl Row {
    pub fn new(window: &mut Window, cx: &mut App) -> Self {
        let name = cx.new(|cx| InputState::new(window, cx).placeholder("Key"));
        let value = cx.new(|cx| InputState::new(window, cx).placeholder("Value"));

        Self { name, value }
    }

    pub fn render_name(&self, cx: &mut App) -> impl gpui::IntoElement {
        Input::new(&self.name).appearance(false)
    }

    pub fn render_value(&self, cx: &mut App) -> impl gpui::IntoElement {
        Input::new(&self.value).appearance(false)
    }
}

pub struct HeadersTableDelegate {
    headers: IndexSet<Row>,
    columns: [Column; 4],
}

impl HeadersTableDelegate {
    const NAME_COLUMN: &str = "name";
    const VALUE_COLUMN: &str = "value";
    const TOGGLE_COLUMN: &str = "toggle";
    const DELETE_COLUMN: &str = "delete";

    pub fn new() -> Self {
        Self {
            headers: IndexSet::new(),
            columns: [
                Column::new(Self::NAME_COLUMN, "").fixed_left(),
                Column::new(Self::VALUE_COLUMN, ""),
                Column::new(Self::TOGGLE_COLUMN, ""),
                Column::new(Self::DELETE_COLUMN, ""),
            ],
        }
    }

    pub fn create_row(&mut self, window: &mut Window, cx: &mut App) {
        self.headers.insert(Row::new(window, cx));
    }
}

impl TableDelegate for HeadersTableDelegate {
    fn columns_count(&self, cx: &gpui::App) -> usize {
        self.columns.len()
    }

    fn rows_count(&self, cx: &gpui::App) -> usize {
        self.headers.len()
    }

    fn column(&self, col_ix: usize, cx: &gpui::App) -> &gpui_component::table::Column {
        &self.columns[col_ix]
    }

    fn render_td(
        &self,
        row_ix: usize,
        col_ix: usize,
        window: &mut gpui::Window,
        cx: &mut App,
    ) -> impl gpui::IntoElement {
        let column = &self.columns[col_ix];
        let row = self.headers.get_index(row_ix);

        if let Some(row) = row {
            div().map(|parent| match column.key.as_ref() {
                Self::NAME_COLUMN => parent.child(row.render_name(cx)),
                Self::VALUE_COLUMN => parent.child(row.render_value(cx)),
                _ => parent,
            })
        } else {
            div()
        }
    }
}
