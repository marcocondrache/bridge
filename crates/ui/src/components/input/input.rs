use std::ops::Range;

use gpui::{
    App, AppContext, Bounds, ClipboardItem, Context, CursorStyle, Element, ElementId,
    ElementInputHandler, Entity, EntityInputHandler, FocusHandle, Focusable, GlobalElementId,
    InteractiveElement, IntoElement, KeyBinding, LayoutId, MouseButton, MouseDownEvent,
    MouseMoveEvent, MouseUpEvent, PaintQuad, ParentElement, Pixels, Point, Rems, Render,
    ShapedLine, SharedString, Style, Styled, TextRun, UTF16Selection, UnderlineStyle, Window,
    actions, div, fill, point, prelude::FluentBuilder, px, relative, size,
};
use gpui_component::ActiveTheme;
use ui_component::{Component, titled_group, variant};
use ui_macros::RegisterComponent;

use crate::prelude::*;
use crate::variants::Size;

use super::input_buffer::InputBuffer;

pub(super) const CONTEXT: &str = "Input";

actions!(
    t_input,
    [
        Backspace,
        Delete,
        Left,
        Right,
        SelectLeft,
        SelectRight,
        SelectAll,
        Home,
        End,
        ShowCharacterPalette,
        Paste,
        Cut,
        Copy,
        Quit,
    ]
);

pub fn init(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("backspace", Backspace, Some(CONTEXT)),
        KeyBinding::new("delete", Delete, Some(CONTEXT)),
        KeyBinding::new("shift-left", SelectLeft, Some(CONTEXT)),
        KeyBinding::new("shift-right", SelectRight, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("ctrl-cmd-space", ShowCharacterPalette, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-a", SelectAll, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-a", SelectAll, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-c", Copy, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-c", Copy, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-x", Cut, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-x", Cut, Some(CONTEXT)),
        #[cfg(target_os = "macos")]
        KeyBinding::new("cmd-v", Paste, Some(CONTEXT)),
        #[cfg(not(target_os = "macos"))]
        KeyBinding::new("ctrl-v", Paste, Some(CONTEXT)),
    ]);
}

#[derive(RegisterComponent)]
pub struct Input {
    buffer: InputBuffer,
    placeholder: Option<SharedString>,
    last_layout: Option<ShapedLine>,
    last_bounds: Option<Bounds<Pixels>>,
    size: Size,
    disabled: bool,
    error: Option<SharedString>,
    is_selecting: bool,
    focus_handle: FocusHandle,
}

impl Input {
    pub fn new(cx: &mut App) -> Self {
        Self {
            buffer: InputBuffer::single_line(""),
            placeholder: None,
            last_layout: None,
            last_bounds: None,
            size: Size::default(),
            disabled: false,
            error: None,
            is_selecting: false,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn content(mut self, content: &str) -> Self {
        self.buffer = InputBuffer::single_line(content);
        self
    }

    pub fn get_content(&self) -> String {
        self.buffer.content()
    }

    pub fn is_multiline(&self) -> bool {
        self.buffer.is_multiline()
    }

    fn left(&mut self, _: &Left, _: &mut Window, cx: &mut Context<Self>) {
        self.buffer.move_left();
        cx.notify();
    }

    fn right(&mut self, _: &Right, _: &mut Window, cx: &mut Context<Self>) {
        self.buffer.move_right();
        cx.notify();
    }

    fn select_left(&mut self, _: &SelectLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.buffer.select_left();
        cx.notify();
    }

    fn select_right(&mut self, _: &SelectRight, _: &mut Window, cx: &mut Context<Self>) {
        self.buffer.select_right();
        cx.notify();
    }

    fn select_all(&mut self, _: &SelectAll, _: &mut Window, cx: &mut Context<Self>) {
        self.buffer.select_all();
        cx.notify();
    }

    fn home(&mut self, _: &Home, _: &mut Window, cx: &mut Context<Self>) {
        self.buffer.move_to_start();
        cx.notify();
    }

    fn end(&mut self, _: &End, _: &mut Window, cx: &mut Context<Self>) {
        self.buffer.move_to_end();
        cx.notify();
    }

    fn backspace(&mut self, _: &Backspace, _: &mut Window, cx: &mut Context<Self>) {
        self.buffer.backspace();
        cx.notify();
    }

    fn delete(&mut self, _: &Delete, _: &mut Window, cx: &mut Context<Self>) {
        self.buffer.delete();
        cx.notify();
    }

    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.is_selecting = true;

        let index = self.index_for_mouse_position(event.position);
        if event.modifiers.shift {
            self.buffer.select_to(index);
        } else {
            self.buffer.move_to(index);
        }
        cx.notify();
    }

    fn on_mouse_up(&mut self, _: &MouseUpEvent, _window: &mut Window, _: &mut Context<Self>) {
        self.is_selecting = false;
    }

    fn on_mouse_move(&mut self, event: &MouseMoveEvent, _: &mut Window, cx: &mut Context<Self>) {
        if self.is_selecting {
            self.buffer
                .select_to(self.index_for_mouse_position(event.position));
            cx.notify();
        }
    }

    fn show_character_palette(
        &mut self,
        _: &ShowCharacterPalette,
        window: &mut Window,
        _: &mut Context<Self>,
    ) {
        window.show_character_palette();
    }

    fn paste(&mut self, _: &Paste, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(text) = cx.read_from_clipboard().and_then(|item| item.text()) {
            self.buffer.insert_text(&text);
            cx.notify();
        }
    }

    fn copy(&mut self, _: &Copy, _: &mut Window, cx: &mut Context<Self>) {
        if self.buffer.has_selection() {
            cx.write_to_clipboard(ClipboardItem::new_string(self.buffer.selected_text()));
        }
    }

    fn cut(&mut self, _: &Cut, _: &mut Window, cx: &mut Context<Self>) {
        if self.buffer.has_selection() {
            cx.write_to_clipboard(ClipboardItem::new_string(self.buffer.selected_text()));
            self.buffer.insert_text("");
            cx.notify();
        }
    }

    fn index_for_mouse_position(&self, position: Point<Pixels>) -> usize {
        if self.buffer.is_empty() {
            return 0;
        }

        let (Some(bounds), Some(line)) = (self.last_bounds.as_ref(), self.last_layout.as_ref())
        else {
            return 0;
        };

        if position.y < bounds.top() {
            return 0;
        }
        if position.y > bounds.bottom() {
            return self.buffer.len_chars();
        }

        // ShapedLine uses byte indices, so convert
        let byte_index = line.closest_index_for_x(position.x - bounds.left());
        self.buffer.byte_to_char(byte_index)
    }
}

impl Sizable for Input {
    fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }
}

impl Validatable for Input {
    fn error(mut self, error: Option<impl Into<SharedString>>) -> Self {
        self.error = error.map(|e| e.into());
        self
    }
}

impl Disableable for Input {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Focusable for Input {
    fn focus_handle(&self, _: &gpui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Input {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let has_error = self.error.is_some();

        let height = self.size.height();
        let padding_x = self.size.padding_x();
        let padding_y = self.size.padding_y();

        let base_bg = theme.input;
        let base_fg = theme.foreground;
        let base_border = if has_error {
            theme.danger
        } else {
            theme.border
        };

        div()
            .flex()
            .key_context(CONTEXT)
            .track_focus(&self.focus_handle(cx))
            .items_center()
            .h(height)
            .px(padding_x)
            .py(padding_y)
            .w_full()
            .bg(base_bg)
            .text_color(base_fg)
            .rounded(theme.radius)
            .border_1()
            .border_color(base_border)
            .line_height(Rems(1.25))
            .when(self.disabled, |this| {
                this.opacity(0.5).cursor(CursorStyle::OperationNotAllowed)
            })
            .when(!self.disabled, |this| {
                this.cursor(CursorStyle::IBeam)
                    .on_action(cx.listener(Self::backspace))
                    .on_action(cx.listener(Self::delete))
                    .on_action(cx.listener(Self::left))
                    .on_action(cx.listener(Self::right))
                    .on_action(cx.listener(Self::select_left))
                    .on_action(cx.listener(Self::select_right))
                    .on_action(cx.listener(Self::select_all))
                    .on_action(cx.listener(Self::home))
                    .on_action(cx.listener(Self::end))
                    .on_action(cx.listener(Self::show_character_palette))
                    .on_action(cx.listener(Self::paste))
                    .on_action(cx.listener(Self::cut))
                    .on_action(cx.listener(Self::copy))
                    .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
                    .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
                    .on_mouse_up_out(MouseButton::Left, cx.listener(Self::on_mouse_up))
                    .on_mouse_move(cx.listener(Self::on_mouse_move))
            })
            .when(!has_error, |this| {
                this.focus(|style| style.border_color(theme.primary))
            })
            .child(InputElement { input: cx.entity() })
    }
}

impl EntityInputHandler for Input {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        actual_range: &mut Option<Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<String> {
        let range = self.buffer.range_from_utf16(range_utf16);
        actual_range.replace(self.buffer.range_to_utf16(range.clone()));
        Some(self.buffer.text_for_range(range))
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        Some(UTF16Selection {
            range: self.buffer.range_to_utf16(self.buffer.selected_range()),
            reversed: self.buffer.is_selection_reversed(),
        })
    }

    fn marked_text_range(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Range<usize>> {
        self.buffer
            .marked_range()
            .map(|range| self.buffer.range_to_utf16(range))
    }

    fn unmark_text(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {
        self.buffer.unmark_text();
    }

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let range = range_utf16
            .map(|r| self.buffer.range_from_utf16(r))
            .or(self.buffer.marked_range())
            .unwrap_or_else(|| self.buffer.selected_range());

        self.buffer.replace_range(range, new_text);
        cx.notify();
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range_utf16: Option<Range<usize>>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let range = range_utf16
            .map(|r| self.buffer.range_from_utf16(r))
            .or(self.buffer.marked_range())
            .unwrap_or_else(|| self.buffer.selected_range());

        let new_text_char_len = new_text.chars().count();
        self.buffer.replace_range(range.clone(), new_text);

        if new_text_char_len > 0 {
            self.buffer
                .set_marked_range(Some(range.start..range.start + new_text_char_len));
        } else {
            self.buffer.unmark_text();
        }

        if let Some(new_range_utf16) = new_selected_range_utf16 {
            let new_range = self.buffer.range_from_utf16(new_range_utf16);
            self.buffer.select_to(new_range.end);
        }

        cx.notify();
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        bounds: Bounds<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        let last_layout = self.last_layout.as_ref()?;
        let range_chars = self.buffer.range_from_utf16(range_utf16);

        // Convert char indices to byte indices for ShapedLine
        let range_bytes =
            self.buffer.char_to_byte(range_chars.start)..self.buffer.char_to_byte(range_chars.end);

        Some(Bounds::from_corners(
            point(
                bounds.left() + last_layout.x_for_index(range_bytes.start),
                bounds.top(),
            ),
            point(
                bounds.left() + last_layout.x_for_index(range_bytes.end),
                bounds.bottom(),
            ),
        ))
    }

    fn character_index_for_point(
        &mut self,
        point: gpui::Point<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        let line_point = self.last_bounds?.localize(&point)?;
        let last_layout = self.last_layout.as_ref()?;

        let content = self.buffer.content();
        assert_eq!(last_layout.text, content);

        // ShapedLine returns byte index
        let byte_index = last_layout.index_for_x(point.x - line_point.x)?;
        let char_index = self.buffer.byte_to_char(byte_index);
        Some(self.buffer.offset_to_utf16(char_index))
    }
}

struct InputElement {
    input: Entity<Input>,
}

struct PrepaintState {
    line: Option<ShapedLine>,
    cursor: Option<PaintQuad>,
    selection: Option<PaintQuad>,
}

impl IntoElement for InputElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for InputElement {
    type RequestLayoutState = ();
    type PrepaintState = PrepaintState;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.size.width = relative(1.).into();
        style.size.height = window.line_height().into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let input = self.input.read(cx);
        let theme = cx.theme();
        let placeholder = input.placeholder.clone();
        let content = input.buffer.content();
        let selected_range = input.buffer.selected_range();
        let cursor = input.buffer.cursor_offset();
        let text_style = window.text_style();

        let (display_text, text_color) = if content.is_empty() {
            (placeholder.unwrap_or_default(), theme.muted_foreground)
        } else {
            (content.into(), theme.foreground)
        };

        let run = TextRun {
            len: display_text.len(),
            font: text_style.font(),
            color: text_color,
            background_color: None,
            underline: None,
            strikethrough: None,
        };

        let runs = if let Some(marked_range) = input.buffer.marked_range() {
            // Convert char indices to byte indices for TextRun
            let marked_range_bytes = input.buffer.char_to_byte(marked_range.start)
                ..input.buffer.char_to_byte(marked_range.end);

            vec![
                TextRun {
                    len: marked_range_bytes.start,
                    ..run.clone()
                },
                TextRun {
                    len: marked_range_bytes.end - marked_range_bytes.start,
                    underline: Some(UnderlineStyle {
                        color: Some(run.color),
                        thickness: px(1.0),
                        wavy: false,
                    }),
                    ..run.clone()
                },
                TextRun {
                    len: display_text.len() - marked_range_bytes.end,
                    ..run
                },
            ]
            .into_iter()
            .filter(|run| run.len > 0)
            .collect()
        } else {
            vec![run]
        };

        let font_size = input.size.font_size();
        let line = window
            .text_system()
            .shape_line(display_text, font_size, &runs, None);

        // Convert char indices to byte indices for line positioning
        let cursor_byte = input.buffer.char_to_byte(cursor);
        let cursor_pos = line.x_for_index(cursor_byte);

        let theme = cx.theme();
        let (selection, cursor) = if selected_range.is_empty() {
            (
                None,
                Some(fill(
                    Bounds::new(
                        point(bounds.left() + cursor_pos, bounds.top()),
                        size(px(2.), bounds.bottom() - bounds.top()),
                    ),
                    theme.primary,
                )),
            )
        } else {
            let selected_range_bytes = input.buffer.char_to_byte(selected_range.start)
                ..input.buffer.char_to_byte(selected_range.end);

            (
                Some(fill(
                    Bounds::from_corners(
                        point(
                            bounds.left() + line.x_for_index(selected_range_bytes.start),
                            bounds.top(),
                        ),
                        point(
                            bounds.left() + line.x_for_index(selected_range_bytes.end),
                            bounds.bottom(),
                        ),
                    ),
                    theme.primary.opacity(0.2),
                )),
                None,
            )
        };
        PrepaintState {
            line: Some(line),
            cursor,
            selection,
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let focus_handle = self.input.read(cx).focus_handle.clone();
        window.handle_input(
            &focus_handle,
            ElementInputHandler::new(bounds, self.input.clone()),
            cx,
        );
        if let Some(selection) = prepaint.selection.take() {
            window.paint_quad(selection)
        }
        let line = prepaint.line.take().unwrap();
        line.paint(bounds.origin, window.line_height(), window, cx)
            .unwrap();

        if focus_handle.is_focused(window)
            && let Some(cursor) = prepaint.cursor.take()
        {
            window.paint_quad(cursor);
        }

        self.input.update(cx, |input, _cx| {
            input.last_layout = Some(line);
            input.last_bounds = Some(bounds);
        });
    }
}

impl Component for Input {
    fn description() -> Option<&'static str> {
        Some(
            "A text input component with support for single-line text editing, selection, clipboard operations, and validation states",
        )
    }

    fn showcase(_window: &mut gpui::Window, cx: &mut gpui::App) -> Option<gpui::AnyElement> {
        Some(
            div()
                .v_flex()
                .gap_6()
                .children(vec![
                    titled_group(
                        "Sizes",
                        vec![
                            variant(
                                "Small",
                                cx.new(|cx| {
                                    Input::new(cx)
                                        .placeholder("Small input")
                                        .content("Small text")
                                        .small()
                                })
                                .into_any_element(),
                            ),
                            variant(
                                "Default",
                                cx.new(|cx| {
                                    Input::new(cx)
                                        .placeholder("Default input")
                                        .content("Default text")
                                })
                                .into_any_element(),
                            ),
                            variant(
                                "Medium",
                                cx.new(|cx| {
                                    Input::new(cx)
                                        .placeholder("Medium input")
                                        .content("Medium text")
                                        .medium()
                                })
                                .into_any_element(),
                            ),
                            variant(
                                "Large",
                                cx.new(|cx| {
                                    Input::new(cx)
                                        .placeholder("Large input")
                                        .content("Large text")
                                        .large()
                                })
                                .into_any_element(),
                            ),
                        ],
                    ),
                    titled_group(
                        "States",
                        vec![
                            variant(
                                "Default",
                                cx.new(|cx| {
                                    Input::new(cx)
                                        .placeholder("Type something...")
                                        .content("Normal state")
                                })
                                .into_any_element(),
                            ),
                            variant(
                                "Disabled",
                                cx.new(|cx| {
                                    Input::new(cx)
                                        .placeholder("Disabled input")
                                        .content("Can't edit this")
                                        .disabled(true)
                                })
                                .into_any_element(),
                            ),
                            variant(
                                "With Placeholder",
                                cx.new(|cx| Input::new(cx).placeholder("Enter your email..."))
                                    .into_any_element(),
                            ),
                        ],
                    ),
                    titled_group(
                        "Validation States",
                        vec![
                            variant(
                                "Valid",
                                cx.new(|cx| {
                                    Input::new(cx)
                                        .placeholder("Email")
                                        .content("user@example.com")
                                        .valid(true)
                                })
                                .into_any_element(),
                            ),
                            variant(
                                "Error",
                                cx.new(|cx| {
                                    Input::new(cx)
                                        .placeholder("Email")
                                        .content("invalid-email")
                                        .error(Some("Invalid email format"))
                                })
                                .into_any_element(),
                            ),
                            variant(
                                "Error (Empty)",
                                cx.new(|cx| {
                                    Input::new(cx)
                                        .placeholder("Required field")
                                        .error(Some("This field is required"))
                                })
                                .into_any_element(),
                            ),
                        ],
                    ),
                    titled_group(
                        "Common Use Cases",
                        vec![
                            variant(
                                "Email",
                                cx.new(|cx| Input::new(cx).placeholder("email@example.com"))
                                    .into_any_element(),
                            ),
                            variant(
                                "Search",
                                cx.new(|cx| Input::new(cx).placeholder("Search..."))
                                    .into_any_element(),
                            ),
                            variant(
                                "Name",
                                cx.new(|cx| Input::new(cx).placeholder("Full name"))
                                    .into_any_element(),
                            ),
                        ],
                    ),
                ])
                .into_any_element(),
        )
    }
}
