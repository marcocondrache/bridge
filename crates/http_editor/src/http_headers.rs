use gpui::{ParentElement, div};
use gpui_component::{
    label::Label,
    table::{Column, TableDelegate},
};
use http_client::HeaderMap;

pub struct HttpHeaders {
    headers: Option<HeaderMap>,
    columns: Vec<Column>,
}

impl HttpHeaders {
    pub fn new() -> Self {
        Self {
            headers: None,
            columns: Self::static_columns(),
        }
    }

    pub fn new_editable() -> Self {
        Self {
            headers: None,
            columns: Self::editable_columns(),
        }
    }

    pub fn set_headers(&mut self, headers: HeaderMap) {
        self.headers = Some(headers);
    }

    fn static_columns() -> Vec<Column> {
        vec![Column::new("name", "Name"), Column::new("value", "Value")]
    }

    fn editable_columns() -> Vec<Column> {
        vec![Column::new("name", "Name"), Column::new("value", "Value")]
    }
}

impl TableDelegate for HttpHeaders {
    fn columns_count(&self, cx: &gpui::App) -> usize {
        self.columns.len()
    }

    fn rows_count(&self, cx: &gpui::App) -> usize {
        self.headers.as_ref().map(|x| x.len()).unwrap_or(0)
    }

    fn column(&self, col_ix: usize, cx: &gpui::App) -> &gpui_component::table::Column {
        &self.columns[col_ix]
    }

    fn render_td(
        &self,
        row_ix: usize,
        col_ix: usize,
        window: &mut gpui::Window,
        cx: &mut gpui::Context<gpui_component::table::Table<Self>>,
    ) -> impl gpui::IntoElement {
        let column = &self.columns[col_ix];
        let row = self.headers.as_ref().unwrap().iter().nth(row_ix).unwrap();
        let raw = row.1.to_str().unwrap().to_string();

        match column.key.as_ref() {
            "name" => div().child(Label::new(row.0.to_string())),
            "value" => div().child(Label::new(raw)),
            _ => div(),
        }
    }
}
