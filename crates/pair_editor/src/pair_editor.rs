use gpui::{
    AbsoluteLength, App, Context, CursorStyle, DefiniteLength, Entity, FocusHandle, Focusable,
    InteractiveElement, IntoElement, MouseButton, MouseDownEvent, ParentElement, Render,
    SharedString, Styled, Subscription, Window, actions, div, prelude::FluentBuilder, px, rems,
};
use gpui_component::ActiveTheme;
use strum::{Display, IntoStaticStr};
use ui::{
    components::{button::Button, checkbox::Checkbox, input::Input, table::Table},
    prelude::*,
    traits::Toggleable,
};

actions!(pair_editor, [AddPair, DeletePair]);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IntoStaticStr, Display)]
pub enum PairItem {
    Key,
    Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IntoStaticStr, Display)]
pub enum PairCell {
    Toggle,
    Item(PairItem),
    Remove,
}

impl PairCell {
    pub fn all() -> [PairCell; 4] {
        [
            PairCell::Toggle,
            PairCell::Item(PairItem::Key),
            PairCell::Item(PairItem::Value),
            PairCell::Remove,
        ]
    }
}

#[derive(Debug, Clone)]
pub struct Pair {
    pub hidden: bool,
    pub enabled: bool,
    pub items: (SharedString, SharedString),
}

impl Pair {
    pub fn new() -> Self {
        Self {
            hidden: false,
            enabled: true,
            items: (SharedString::default(), SharedString::default()),
        }
    }

    pub fn with_key_value(key: impl Into<SharedString>, value: impl Into<SharedString>) -> Self {
        Self {
            hidden: false,
            enabled: true,
            items: (key.into(), value.into()),
        }
    }

    pub fn set_key(&mut self, key: impl Into<SharedString>) {
        self.items.0 = key.into();
    }

    pub fn key(&self) -> &SharedString {
        &self.items.0
    }

    pub fn set_value(&mut self, value: impl Into<SharedString>) {
        self.items.1 = value.into();
    }

    pub fn value(&self) -> &SharedString {
        &self.items.1
    }

    pub fn toggle_enable(&mut self) {
        self.enabled = !self.enabled;
    }

    pub fn toggle_hidden(&mut self) {
        self.hidden = !self.hidden;
    }

    pub fn set_item(&mut self, item: PairItem, value: impl Into<SharedString>) {
        match item {
            PairItem::Key => self.items.0 = value.into(),
            PairItem::Value => self.items.1 = value.into(),
        }
    }

    pub fn get_item(&self, item: PairItem) -> &SharedString {
        match item {
            PairItem::Key => &self.items.0,
            PairItem::Value => &self.items.1,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.0.is_empty() && self.items.1.is_empty()
    }

    pub fn has_content(&self) -> bool {
        !self.items.0.is_empty() || !self.items.1.is_empty()
    }
}

pub struct PairEditor {
    pairs: Vec<Pair>,
    auto_create: bool,
    input: Entity<Input>,
    active_cell: Option<(usize, PairItem)>,
    focus_handle: FocusHandle,
    _focus_subscription: Subscription,
}

impl PairEditor {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| Input::new(cx).large());

        let focus_handle = cx.focus_handle();
        let focus_subscription =
            cx.on_focus_out(&input.focus_handle(cx), window, |this, _, _window, cx| {
                this.blur_item(cx);
            });

        Self {
            pairs: vec![Pair::new()],
            auto_create: true,
            active_cell: None,
            input,
            focus_handle,
            _focus_subscription: focus_subscription,
        }
    }

    pub fn auto_create(mut self, enabled: bool) -> Self {
        self.auto_create = enabled;
        self
    }

    pub fn pairs(mut self, pairs: impl Iterator<Item = Pair>) -> Self {
        self.pairs = pairs.collect();
        self
    }

    pub fn get_pairs(&self) -> Vec<Pair> {
        self.pairs
            .iter()
            .filter(|p| !p.is_empty())
            .cloned()
            .collect()
    }

    pub fn get_enabled_pairs(&self) -> Vec<Pair> {
        self.pairs
            .iter()
            .filter(|p| p.enabled && !p.is_empty())
            .cloned()
            .collect()
    }

    fn focus_item(
        &mut self,
        index: usize,
        item: PairItem,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.save_active_input(cx);

        let content = self.pairs[index].get_item(item).to_string();

        self.input.update(cx, |this, _| this.set_content(&content));
        self.active_cell = Some((index, item));

        window.focus(&self.input.focus_handle(cx));

        cx.notify();
    }

    fn save_active_input(&mut self, cx: &mut Context<Self>) {
        if let Some((index, item)) = self.active_cell {
            if index < self.pairs.len() {
                let content = self.input.read(cx).get_content();
                self.pairs[index].set_item(item, content);
            }
        }
    }

    fn blur_item(&mut self, cx: &mut Context<Self>) {
        self.save_active_input(cx);
        self.check_auto_create();

        self.active_cell = None;

        cx.notify();
    }

    fn check_auto_create(&mut self) {
        if !self.auto_create {
            return;
        }

        if self.pairs.iter().all(|p| p.has_content()) {
            self.pairs.push(Pair::new());
        }
    }

    fn add_pair(&mut self, _: &AddPair, _: &mut Window, cx: &mut Context<Self>) {
        self.pairs.push(Pair::new());

        cx.notify();
    }

    fn delete_pair(&mut self, index: usize, cx: &mut Context<Self>) {
        if self.pairs.len() <= 1 {
            self.pairs[0] = Pair::new();
            self.blur_item(cx);

            cx.notify();
            return;
        }

        if let Some((active_row, _)) = self.active_cell {
            if active_row == index {
                self.blur_item(cx);
            }
        }

        self.pairs.remove(index);
        cx.notify();
    }

    fn toggle_enabled(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.pairs.len() {
            self.pairs[index].toggle_enable();
            cx.notify();
        }
    }

    fn render_cell(
        &mut self,
        index: usize,
        cell: PairCell,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let pair = &self.pairs[index];

        let id: (&'static str, usize) = (cell.into(), index);

        match cell {
            PairCell::Toggle => {
                div()
                    .id(id)
                    .size_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(Checkbox::new("toggle").toggle_state(pair.enabled).on_click(
                        cx.listener(move |this, _, _window, cx| {
                            this.toggle_enabled(index, cx);
                        }),
                    ))
            }
            PairCell::Remove => div().id(id).flex().items_center().justify_center().child(
                Button::new(id)
                    .label("×")
                    .small()
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.delete_pair(index, cx);
                    })),
            ),
            PairCell::Item(item) => {
                let is_active = self.active_cell == Some((index, item));

                if is_active {
                    div().id(id).size_full().px_1().child(self.input.clone())
                } else {
                    div()
                        .id(id)
                        .size_full()
                        .min_h(rems(2.0))
                        .border_1()
                        .border_color(theme.border)
                        .rounded(px(4.))
                        .px_2()
                        .py_1()
                        .cursor(CursorStyle::IBeam)
                        .hover(|this| this.border_color(theme.muted_foreground))
                        .when_else(
                            pair.enabled,
                            |this| this.bg(theme.input),
                            |this| {
                                this.bg(theme.muted)
                                    .text_color(theme.muted_foreground)
                                    .opacity(0.3)
                            },
                        )
                        .on_click(cx.listener(move |this, _, window, cx| {
                            this.focus_item(index, item, window, cx);
                        }))
                        .child(
                            div()
                                .when(pair.get_item(item).is_empty(), |this| {
                                    this.text_color(theme.muted_foreground)
                                        .child(item.to_string())
                                })
                                .when(!pair.get_item(item).is_empty(), |this| {
                                    this.child(pair.get_item(item).clone())
                                }),
                        )
                }
            }
        }
    }
}

impl Focusable for PairEditor {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for PairEditor {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let rows = self
            .pairs
            .clone()
            .iter()
            .enumerate()
            .map(|(idx, _)| {
                [
                    self.render_cell(idx, PairCell::Toggle, cx)
                        .into_any_element(),
                    self.render_cell(idx, PairCell::Item(PairItem::Key), cx)
                        .into_any_element(),
                    self.render_cell(idx, PairCell::Item(PairItem::Value), cx)
                        .into_any_element(),
                    self.render_cell(idx, PairCell::Remove, cx)
                        .into_any_element(),
                ]
            })
            .collect::<Vec<_>>();

        div()
            .id("pair-editor")
            .track_focus(&self.focus_handle)
            .size_full()
            .child(
                Table::<4>::new()
                    .header(["", "Key", "Value", ""])
                    .column_widths([
                        DefiniteLength::Absolute(AbsoluteLength::Pixels(px(48.))),
                        DefiniteLength::Fraction(0.50),
                        DefiniteLength::Fraction(0.50),
                        DefiniteLength::Absolute(AbsoluteLength::Pixels(px(48.))),
                    ])
                    .rows(rows),
            )
    }
}
