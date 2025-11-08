use gpui::{App, ParentElement, div};
use gpui_component::{
    label::Label,
    table::{Column, TableDelegate},
};
use indexmap::IndexMap;

pub struct QueryTableDelegate {
    params: IndexMap<String, String>,
    columns: [Column; 2],
}

impl QueryTableDelegate {
    const NAME_COLUMN: &str = "name";
    const VALUE_COLUMN: &str = "value";

    pub fn new() -> Self {
        Self {
            params: IndexMap::new(),
            columns: [
                Column::new(Self::NAME_COLUMN, "Name"),
                Column::new(Self::VALUE_COLUMN, "Value"),
            ],
        }
    }
}

impl TableDelegate for QueryTableDelegate {
    fn columns_count(&self, cx: &gpui::App) -> usize {
        self.columns.len()
    }

    fn rows_count(&self, cx: &gpui::App) -> usize {
        self.params.len()
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
        let row = &self.params.get_index(row_ix);

        if let Some(row) = row {
            match column.key.as_ref() {
                Self::NAME_COLUMN => div().child(Label::new(row.0)),
                Self::VALUE_COLUMN => div().child(Label::new(row.1)),
                _ => div(),
            }
        } else {
            div()
        }
    }
}
