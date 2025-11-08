use gpui::{IntoElement, Window};
use gpui_component::table::{Column, TableDelegate};

pub trait DynamicItems {
    type Item;

    fn len(&self) -> usize;

    fn get(&self, index: usize) -> Option<&Self::Item>;

    fn render(
        &self,
        item: &Self::Item,
        column: &Column,
        window: &mut Window,
        cx: &mut gpui::App,
    ) -> impl IntoElement;
}

pub struct DynamicDelegate<I> {
    pub items: I,
    columns: Vec<Column>,
}

impl<I> DynamicDelegate<I>
where
    I: Default,
{
    pub fn new(columns: Vec<Column>) -> Self {
        Self {
            items: I::default(),
            columns,
        }
    }
}

impl<I> TableDelegate for DynamicDelegate<I>
where
    I: 'static + DynamicItems,
{
    fn columns_count(&self, _cx: &gpui::App) -> usize {
        self.columns.len()
    }

    fn rows_count(&self, _cx: &gpui::App) -> usize {
        self.items.len()
    }

    fn column(&self, col_ix: usize, _cx: &gpui::App) -> &Column {
        &self.columns[col_ix]
    }

    fn render_td(
        &self,
        row_ix: usize,
        col_ix: usize,
        window: &mut gpui::Window,
        cx: &mut gpui::App,
    ) -> impl gpui::IntoElement {
        let item = self.items.get(row_ix);
        let column = &self.columns[col_ix];

        self.items.render(item.unwrap(), column, window, cx)
    }
}
