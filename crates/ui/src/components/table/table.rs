use std::ops::Range;

use gpui::{
    AnyElement, App, Div, ElementId, FocusHandle, InteractiveElement, IntoElement, Length,
    ListHorizontalSizingBehavior, ListSizingBehavior, ParentElement, RenderOnce, Styled,
    UniformListScrollHandle, WeakEntity, Window, div, prelude::FluentBuilder, uniform_list,
};
use gpui_component::ActiveTheme;

use crate::traits::styled_ext::StyledExt;

enum TableRows<const COLS: usize> {
    Eager(Vec<[AnyElement; COLS]>),
    Lazy {
        count: usize,
        render_fn: Box<dyn Fn(Range<usize>, &mut Window, &mut App) -> Vec<[AnyElement; COLS]>>,
    },
}

impl<const COLS: usize> TableRows<COLS> {
    fn len(&self) -> usize {
        match self {
            TableRows::Eager(rows) => rows.len(),
            TableRows::Lazy { count, .. } => *count,
        }
    }
}

#[derive(IntoElement)]
pub struct Table<const COLS: usize> {
    headers: Option<[AnyElement; COLS]>,
    width: Option<Length>,
    rows: TableRows<COLS>,
}

impl<const COLS: usize> Table<COLS> {
    pub fn new() -> Self {
        Self {
            width: None,
            headers: None,
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

    fn render_header(headers: [AnyElement; COLS], cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();

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
                    .enumerate()
                    .map(|(index, header)| Table::<COLS>::base_cell().id(index).child(header)),
            )
    }

    fn render_row(index: usize, items: [AnyElement; COLS], count: usize) -> impl IntoElement {
        let is_last = index == count - 1;

        let row = div()
            .h_flex()
            .id(("table_row", index))
            .w_full()
            .justify_between()
            .when(!is_last, |this| this.border_b_1())
            .children(
                items
                    .into_iter()
                    .map(|c| Table::<COLS>::base_cell().child(c)),
            );

        div().size_full().child(row)
    }

    fn base_cell() -> Div {
        div()
            .px_1p5()
            .flex_1()
            .whitespace_nowrap()
            .text_ellipsis()
            .overflow_hidden()
    }
}

impl<const COLS: usize> RenderOnce for Table<COLS> {
    fn render(self, window: &mut gpui::Window, cx: &mut gpui::App) -> impl IntoElement {
        let width = self.width;
        let table =
            div()
                .when_some(width, |this, width| this.w(width))
                .h_full()
                .v_flex()
                .when_some(self.headers, |parent, headers| {
                    parent.child(Table::render_header(headers, cx))
                })
                .child(
                    div().flex_grow().w_full().relative().overflow_hidden().map(
                        |parent| match self.rows {
                            TableRows::Eager(items) => {
                                let count = items.len();

                                print!("{} rows", count);

                                parent.child(
                                    div().children(
                                        items.into_iter().enumerate().map(|(index, row)| {
                                            Table::render_row(index, row, count)
                                        }),
                                    ),
                                )
                            }
                            TableRows::Lazy { count, render_fn } => parent.child(
                                uniform_list("", count, move |range, window, cx| {
                                    let elements = render_fn(range.clone(), window, cx);

                                    elements
                                        .into_iter()
                                        .zip(range)
                                        .map(|(row, index)| Table::render_row(index, row, count))
                                        .collect()
                                })
                                .size_full()
                                .flex_grow()
                                .with_sizing_behavior(ListSizingBehavior::Auto)
                                .when_else(
                                    width.is_some(),
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
                        },
                    ),
                );

        table
    }
}
