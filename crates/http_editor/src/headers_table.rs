use std::str::FromStr;

use anyhow::{Ok, Result};

use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, WeakEntity, Window, div,
    prelude::FluentBuilder,
};
use gpui_component::{
    input::{Input, InputState},
    table::{Column, TableState},
};
use http::{HeaderName, HeaderValue};
use indexmap::{IndexSet, set::MutableValues};

use crate::dynamic_delegate::{DynamicDelegate, DynamicItems};

type HeadersMap = IndexSet<(Option<HeaderName>, HeaderValue)>;
type HeadersRows = IndexSet<HeadersRow>;

pub type HeadersTable = TableState<DynamicDelegate<HeadersMap>>;
pub type HeadersTableEditor = TableState<DynamicDelegate<HeadersRows>>;

pub fn headers_table_editor(
    window: &mut gpui::Window,
    cx: &mut Context<HeadersTableEditor>,
) -> HeadersTableEditor {
    TableState::new(
        DynamicDelegate::new(vec![
            Column::new(DynamicDelegate::<HeadersRows>::KEY_COLUMN, "Name"),
            Column::new(DynamicDelegate::<HeadersRows>::VALUE_COLUMN, "Value"),
        ]),
        window,
        cx,
    )
}

pub fn headers_table(window: &mut gpui::Window, cx: &mut Context<HeadersTable>) -> HeadersTable {
    TableState::new(
        DynamicDelegate::new(vec![
            Column::new(DynamicDelegate::<HeadersMap>::KEY_COLUMN, "Name"),
            Column::new(DynamicDelegate::<HeadersMap>::VALUE_COLUMN, "Value"),
        ]),
        window,
        cx,
    )
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HeadersRow {
    key: Entity<InputState>,
    value: Entity<InputState>,
    table: WeakEntity<HeadersTableEditor>,
    enabled: bool,
}

impl HeadersRow {
    pub fn new(window: &mut Window, cx: &mut Context<HeadersTableEditor>) -> Self {
        let key = cx.new(|cx| InputState::new(window, cx).placeholder("Key"));
        let value = cx.new(|cx| InputState::new(window, cx).placeholder("Value"));

        Self {
            key,
            value,
            enabled: true,
            table: cx.entity().downgrade(),
        }
    }

    pub fn render_key(&self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        Input::new(&self.key).appearance(false)
    }

    pub fn render_value(&self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        Input::new(&self.value).appearance(false)
    }

    pub fn get_header(&self, cx: &App) -> Result<(HeaderName, HeaderValue)> {
        let key = self.key.read(cx).value();
        let value = self.value.read(cx).value();

        Ok((HeaderName::from_str(&key)?, HeaderValue::from_str(&value)?))
    }
}

impl DynamicDelegate<HeadersRows> {
    const KEY_COLUMN: &str = "key";
    const VALUE_COLUMN: &str = "value";
    const TOGGLE_COLUMN: &str = "toggle";
    const DELETE_COLUMN: &str = "delete";

    pub fn create_row(&mut self, window: &mut Window, cx: &mut Context<HeadersTableEditor>) {
        self.items.insert(HeadersRow::new(window, cx));
    }

    pub fn remove_row(&mut self, index: usize) {
        self.items.shift_remove_index(index);
    }

    pub fn toggle_row(&mut self, index: usize) {
        if let Some(row) = self.items.get_index_mut2(index) {
            row.enabled = !row.enabled;
        }
    }

    pub fn get_headers(&self, window: &mut Window, cx: &App) -> http::HeaderMap {
        http::HeaderMap::from_iter(
            self.items
                .clone()
                .iter()
                .map(|row| row.get_header(cx).unwrap()),
        )
    }
}

impl DynamicItems for HeadersRows {
    type Item = HeadersRow;

    fn len(&self) -> usize {
        self.len()
    }

    fn get(&self, index: usize) -> Option<&Self::Item> {
        self.get_index(index)
    }

    fn render(
        &self,
        item: &Self::Item,
        column: &gpui_component::table::Column,
        window: &mut gpui::Window,
        cx: &mut gpui::App,
    ) -> impl gpui::IntoElement {
        div().map(|parent| match column.key.as_ref() {
            DynamicDelegate::<IndexSet<HeadersRow>>::KEY_COLUMN => {
                parent.child(item.render_key(window, cx))
            }
            DynamicDelegate::<IndexSet<HeadersRow>>::VALUE_COLUMN => {
                parent.child(item.render_value(window, cx))
            }
            _ => parent,
        })
    }
}

impl DynamicDelegate<HeadersMap> {
    const KEY_COLUMN: &str = "key";
    const VALUE_COLUMN: &str = "value";

    pub fn set_headers(&mut self, headers: http::HeaderMap) {
        self.items = IndexSet::from_iter(headers)
    }
}

// TODO: A map would be better
impl DynamicItems for IndexSet<(Option<HeaderName>, HeaderValue)> {
    type Item = (Option<HeaderName>, HeaderValue);

    fn len(&self) -> usize {
        self.len()
    }

    fn get(&self, index: usize) -> Option<&Self::Item> {
        self.get_index(index)
    }

    fn render(
        &self,
        item: &Self::Item,
        column: &Column,
        window: &mut Window,
        cx: &mut gpui::App,
    ) -> impl IntoElement {
        div().map(|parent| match column.key.as_ref() {
            DynamicDelegate::<HeadersMap>::KEY_COLUMN => {
                parent.when_some(item.0.clone(), |this, key| this.child(key.to_string()))
            }
            DynamicDelegate::<HeadersMap>::VALUE_COLUMN => parent
                .when_some(item.1.to_str().ok(), |this, value| {
                    this.child(value.to_string())
                }),
            _ => parent,
        })
    }
}
