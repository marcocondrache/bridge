use gpui::{ParentElement, div, prelude::FluentBuilder};
use gpui_component::{
    label::Label,
    table::{Column, TableDelegate},
};
use http_client::{HeaderMap, HeaderName, HeaderValue};
use indexmap::IndexMap;
use smallvec::SmallVec;

pub struct HeadersTableDelegate {
    headers: IndexMap<Option<HeaderName>, HeaderValue>,
    columns: SmallVec<[Column; 4]>,
}

impl HeadersTableDelegate {
    const NAME_COLUMN: &str = "name";
    const VALUE_COLUMN: &str = "value";

    pub fn new(header_map: HeaderMap) -> Self {
        Self {
            headers: IndexMap::from_iter(header_map),
            columns: SmallVec::from_iter(Self::static_columns()),
        }
    }

    pub fn new_editable() -> Self {
        Self {
            headers: IndexMap::new(),
            columns: SmallVec::from_iter(Self::editable_columns()),
        }
    }

    pub fn set_headers(&mut self, headers: HeaderMap) {
        self.headers = IndexMap::from_iter(headers);
    }

    fn static_columns() -> [Column; 2] {
        [
            Column::new(Self::NAME_COLUMN, "Name"),
            Column::new(Self::VALUE_COLUMN, "Value"),
        ]
    }

    fn editable_columns() -> [Column; 2] {
        [
            Column::new(Self::NAME_COLUMN, "Name"),
            Column::new(Self::VALUE_COLUMN, "Value"),
        ]
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
        cx: &mut gpui::Context<gpui_component::table::Table<Self>>,
    ) -> impl gpui::IntoElement {
        let column = &self.columns[col_ix];
        let row = self.headers.get_index(row_ix);

        if let Some((name, value)) = row {
            match column.key.as_ref() {
                Self::NAME_COLUMN => div().when_some(name.clone(), |this, name| {
                    this.child(Label::new(name.to_string()))
                }),
                Self::VALUE_COLUMN => div().when_some(value.to_str().ok(), |this, value| {
                    this.child(Label::new(value.to_string()))
                }),
                _ => div(),
            }
        } else {
            div()
        }
    }
}
