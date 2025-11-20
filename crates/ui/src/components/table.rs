use std::ops::Range;

use gpui::{
    AnyElement, App, DefiniteLength, Div, InteractiveElement, IntoElement, Length,
    ListHorizontalSizingBehavior, ListSizingBehavior, ParentElement, RenderOnce, Styled, Window,
    div, prelude::FluentBuilder, uniform_list,
};
use gpui_component::ActiveTheme;
use ui_component::{Component, titled_group, variant};

use crate::traits::styled_ext::StyledExt;

pub struct TableWidths<const COLS: usize> {
    initial: [DefiniteLength; COLS],
}

impl<const COLS: usize> TableWidths<COLS> {
    pub fn new(widths: [impl Into<DefiniteLength>; COLS]) -> Self {
        let widths = widths.map(Into::into);

        TableWidths { initial: widths }
    }

    fn lengths(&self, _cx: &App) -> [Length; COLS] {
        self.initial.map(Length::Definite)
    }
}

enum TableRows<const COLS: usize> {
    Eager(Vec<[AnyElement; COLS]>),
    Lazy {
        count: usize,
        render_fn: Box<dyn Fn(Range<usize>, &mut Window, &mut App) -> Vec<[AnyElement; COLS]>>,
    },
}

#[derive(IntoElement)]
pub struct Table<const COLS: usize> {
    headers: Option<[AnyElement; COLS]>,
    column_widths: Option<TableWidths<COLS>>,
    width: Option<Length>,
    rows: TableRows<COLS>,
}

impl<const COLS: usize> Table<COLS> {
    pub fn new() -> Self {
        Self {
            width: None,
            headers: None,
            column_widths: None,
            rows: TableRows::Eager(Vec::new()),
        }
    }

    pub fn list<F>(mut self, count: usize, render_fn: F) -> Self
    where
        F: Fn(Range<usize>, &mut Window, &mut App) -> Vec<[AnyElement; COLS]> + 'static,
    {
        self.rows = TableRows::Lazy {
            count,
            render_fn: Box::new(render_fn),
        };
        self
    }

    pub fn header(mut self, headers: [impl IntoElement; COLS]) -> Self {
        self.headers = Some(headers.map(IntoElement::into_any_element));
        self
    }

    pub fn column_widths(mut self, widths: [impl Into<DefiniteLength>; COLS]) -> Self {
        if self.column_widths.is_none() {
            self.column_widths = Some(TableWidths::new(widths));
        }

        self
    }

    pub fn row(mut self, row: [impl IntoElement; COLS]) -> Self {
        if let TableRows::Eager(rows) = &mut self.rows {
            rows.push(row.map(IntoElement::into_any_element));
        }

        self
    }

    pub fn rows(mut self, rows: impl IntoIterator<Item = [impl IntoElement; COLS]>) -> Self {
        if let TableRows::Eager(existing_rows) = &mut self.rows {
            existing_rows.extend(
                rows.into_iter()
                    .map(|row| row.map(IntoElement::into_any_element)),
            );
        }

        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    fn render_header(
        headers: [AnyElement; COLS],
        widths: Option<[Length; COLS]>,
        cx: &mut App,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let widths = widths.map_or([None; COLS], |w| w.map(Some));

        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .p_2()
            .border_b_1()
            .border_color(theme.border)
            .children(
                headers
                    .into_iter()
                    .zip(widths)
                    .map(|(header, width)| Table::<COLS>::base_cell(width).child(header)),
            )
    }

    fn render_row(
        index: usize,
        items: [AnyElement; COLS],
        widths: Option<[Length; COLS]>,
        count: usize,
    ) -> impl IntoElement {
        let is_last = index == count - 1;
        let widths = widths.map_or([None; COLS], |w| w.map(Some));

        let row =
            div()
                .h_flex()
                .id(("table_row", index))
                .w_full()
                .justify_between()
                .when(!is_last, |this| this.border_b_1())
                .children(items.into_iter().zip(widths).map(|(cell, width)| {
                    Table::<COLS>::base_cell(width).px_1().py_0p5().child(cell)
                }));

        div().size_full().child(row)
    }

    fn base_cell(width: Option<Length>) -> Div {
        div()
            .when_none(&width, |this| this.flex_1())
            .when_some(width, |this, width| this.w(width))
            .whitespace_nowrap()
            .text_ellipsis()
            .overflow_hidden()
    }
}

impl<const COLS: usize> RenderOnce for Table<COLS> {
    fn render(self, _window: &mut gpui::Window, cx: &mut gpui::App) -> impl IntoElement {
        let theme = cx.theme();
        let widths = self.column_widths.as_ref().map(|widths| widths.lengths(cx));

        div()
            .when_some(self.width, |this, width| this.w(width))
            .h_full()
            .v_flex()
            .border_1()
            .border_color(theme.border)
            .when_some(self.headers, |parent, headers| {
                parent.child(Table::render_header(headers, widths, cx))
            })
            .child(
                div()
                    .flex_grow()
                    .w_full()
                    .relative()
                    .overflow_hidden()
                    .map(|parent| match self.rows {
                        TableRows::Eager(items) => {
                            let count = items.len();

                            parent.child(div().children(
                                items.into_iter().enumerate().map(|(index, row)| {
                                    Table::render_row(index, row, widths, count)
                                }),
                            ))
                        }
                        TableRows::Lazy { count, render_fn } => parent.child(
                            uniform_list("", count, move |range, window, cx| {
                                let elements = render_fn(range.clone(), window, cx);

                                elements
                                    .into_iter()
                                    .zip(range)
                                    .map(|(row, index)| {
                                        Table::render_row(index, row, widths, count)
                                    })
                                    .collect()
                            })
                            .size_full()
                            .flex_grow()
                            .with_sizing_behavior(ListSizingBehavior::Auto)
                            .when_else(
                                self.width.is_some(),
                                |this| {
                                    this.with_horizontal_sizing_behavior(
                                        ListHorizontalSizingBehavior::Unconstrained,
                                    )
                                },
                                |this| {
                                    this.with_horizontal_sizing_behavior(
                                        ListHorizontalSizingBehavior::FitList,
                                    )
                                },
                            ),
                        ),
                    }),
            )
    }
}

impl Component for Table<2> {
    fn name() -> &'static str {
        "Table"
    }

    fn description() -> Option<&'static str> {
        Some(
            "A flexible table component with support for headers, multiple columns, and virtualized rendering for large datasets",
        )
    }

    fn showcase(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        Some(
            div()
                .v_flex()
                .gap_6()
                .children(vec![
                    titled_group(
                        "Basic Tables",
                        vec![
                            variant(
                                "Two Columns with Headers",
                                Table::<2>::new()
                                    .header(["Name", "Email"])
                                    .row(["Alice Johnson", "alice@example.com"])
                                    .row(["Bob Smith", "bob@example.com"])
                                    .row(["Charlie Brown", "charlie@example.com"])
                                    .into_any_element(),
                            ),
                            variant(
                                "Without Headers",
                                Table::<2>::new()
                                    .row(["Item 1", "Value 1"])
                                    .row(["Item 2", "Value 2"])
                                    .row(["Item 3", "Value 3"])
                                    .into_any_element(),
                            ),
                        ],
                    ),
                    titled_group(
                        "Three Columns",
                        vec![variant(
                            "Product Catalog",
                            Table::<3>::new()
                                .header(["Product", "Price", "Stock"])
                                .row(["Laptop", "$999", "In Stock"])
                                .row(["Mouse", "$29", "Low Stock"])
                                .row(["Keyboard", "$79", "In Stock"])
                                .row(["Monitor", "$299", "Out of Stock"])
                                .into_any_element(),
                        )],
                    ),
                    titled_group(
                        "With Custom Width",
                        vec![variant(
                            "Fixed Width Table",
                            div()
                                .h(gpui::rems(12.0))
                                .child(
                                    Table::<2>::new()
                                        .width(gpui::rems(30.0))
                                        .header(["Key", "Value"])
                                        .row(["Configuration", "Production"])
                                        .row(["Environment", "US-West-2"])
                                        .row(["Status", "Active"]),
                                )
                                .into_any_element(),
                        )],
                    ),
                    titled_group(
                        "Lazy Rendering",
                        vec![variant(
                            "Large Dataset (100 rows)",
                            div()
                                .h(gpui::rems(16.0))
                                .child(Table::<2>::new().header(["Index", "Value"]).list(
                                    100,
                                    |range, _window, _cx| {
                                        range
                                            .map(|i| {
                                                [
                                                    format!("Row {}", i + 1).into_any_element(),
                                                    format!("Value {}", (i + 1) * 10)
                                                        .into_any_element(),
                                                ]
                                            })
                                            .collect()
                                    },
                                ))
                                .into_any_element(),
                        )],
                    ),
                ])
                .into_any_element(),
        )
    }
}

#[allow(non_snake_case)]
fn __component_registry_internal_register_Table() {
    ui_component::register_component::<Table<2>>();
}

ui_component::__private::inventory::submit! {
    ui_component::ComponentFn::new(__component_registry_internal_register_Table)
}
