use std::ops::Range;

use ropey::Rope;
use unicode_segmentation::UnicodeSegmentation;

/// Configuration for buffer behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferMode {
    /// Single line mode - newlines are converted to spaces
    SingleLine,
    /// Multiline mode - full text editing with line support
    MultiLine,
}

/// A text buffer that handles editing operations for both single-line and multiline text.
///
/// Uses Ropey for efficient rope-based text storage, providing O(log n) edits and line operations.
/// All operations use char-based indexing (Unicode scalar values) internally.
///
/// The buffer can operate in two modes:
/// - SingleLine: Newlines are automatically converted to spaces
/// - Multiline: Full support for multiple lines with line-aware navigation
pub struct InputBuffer {
    content: Rope,
    selected_range: Range<usize>,
    selection_reversed: bool,
    marked_range: Option<Range<usize>>,
    mode: BufferMode,
}

impl InputBuffer {
    pub fn new(content: &str) -> Self {
        Self::with_mode(content, BufferMode::MultiLine)
    }

    pub fn single_line(content: &str) -> Self {
        Self::with_mode(content, BufferMode::SingleLine)
    }

    pub fn with_mode(content: &str, mode: BufferMode) -> Self {
        let sanitized = Self::sanitize_content(content, mode);
        let content = Rope::from_str(&sanitized);

        Self {
            content,
            selected_range: 0..0,
            selection_reversed: false,
            marked_range: None,
            mode,
        }
    }

    pub fn mode(&self) -> BufferMode {
        self.mode
    }

    pub fn content(&self) -> String {
        self.content.to_string()
    }

    pub fn len_chars(&self) -> usize {
        self.content.len_chars()
    }

    pub fn len_lines(&self) -> usize {
        self.content.len_lines()
    }

    pub fn is_empty(&self) -> bool {
        self.content.len_chars() == 0
    }

    pub fn selected_range(&self) -> Range<usize> {
        self.selected_range.clone()
    }

    pub fn is_selection_reversed(&self) -> bool {
        self.selection_reversed
    }

    pub fn selected_text(&self) -> String {
        self.content.slice(self.selected_range.clone()).to_string()
    }

    pub fn cursor_offset(&self) -> usize {
        if self.selection_reversed {
            self.selected_range.start
        } else {
            self.selected_range.end
        }
    }

    pub fn has_selection(&self) -> bool {
        !self.selected_range.is_empty()
    }

    pub fn marked_range(&self) -> Option<Range<usize>> {
        self.marked_range.clone()
    }

    pub fn set_marked_range(&mut self, range: Option<Range<usize>>) {
        self.marked_range = range;
    }

    pub fn unmark_text(&mut self) {
        self.marked_range = None;
    }

    pub fn move_left(&mut self) {
        if self.has_selection() {
            self.move_to(self.selected_range.start);
        } else {
            let offset = self.cursor_offset();
            if offset > 0 {
                let new_offset = self.previous_boundary(offset);
                self.move_to(new_offset);
            }
        }
    }

    pub fn move_right(&mut self) {
        if self.has_selection() {
            self.move_to(self.selected_range.end);
        } else {
            let offset = self.cursor_offset();
            if offset < self.len_chars() {
                let new_offset = self.next_boundary(offset);
                self.move_to(new_offset);
            }
        }
    }

    pub fn move_up(&mut self) {
        if self.mode == BufferMode::SingleLine {
            self.move_to_start();
            return;
        }

        let cursor = self.cursor_offset();
        if let Some(new_offset) = self.offset_one_line_up(cursor) {
            self.move_to(new_offset);
        }
    }

    pub fn move_down(&mut self) {
        if self.mode == BufferMode::SingleLine {
            self.move_to_end();
            return;
        }

        let cursor = self.cursor_offset();
        if let Some(new_offset) = self.offset_one_line_down(cursor) {
            self.move_to(new_offset);
        }
    }

    pub fn move_to_line_start(&mut self) {
        let cursor = self.cursor_offset();
        let line_start = self.line_start_offset(cursor);
        self.move_to(line_start);
    }

    pub fn move_to_line_end(&mut self) {
        let cursor = self.cursor_offset();
        let line_end = self.line_end_offset(cursor);
        self.move_to(line_end);
    }

    pub fn move_to_start(&mut self) {
        self.move_to(0);
    }

    pub fn move_to_end(&mut self) {
        self.move_to(self.len_chars());
    }

    pub fn move_to(&mut self, offset: usize) {
        let offset = self.clamp_offset(offset);
        self.selected_range = offset..offset;
        self.selection_reversed = false;
    }

    pub fn select_left(&mut self) {
        let cursor = self.cursor_offset();
        if cursor > 0 {
            self.select_to(self.previous_boundary(cursor));
        }
    }

    pub fn select_right(&mut self) {
        let cursor = self.cursor_offset();
        if cursor < self.len_chars() {
            self.select_to(self.next_boundary(cursor));
        }
    }

    pub fn select_up(&mut self) {
        if self.mode == BufferMode::SingleLine {
            self.select_to(0);
            return;
        }

        let cursor = self.cursor_offset();
        if let Some(new_offset) = self.offset_one_line_up(cursor) {
            self.select_to(new_offset);
        }
    }

    pub fn select_down(&mut self) {
        if self.mode == BufferMode::SingleLine {
            self.select_to(self.len_chars());
            return;
        }

        let cursor = self.cursor_offset();
        if let Some(new_offset) = self.offset_one_line_down(cursor) {
            self.select_to(new_offset);
        }
    }

    pub fn select_to_line_start(&mut self) {
        let cursor = self.cursor_offset();
        let line_start = self.line_start_offset(cursor);
        self.select_to(line_start);
    }

    pub fn select_to_line_end(&mut self) {
        let cursor = self.cursor_offset();
        let line_end = self.line_end_offset(cursor);
        self.select_to(line_end);
    }

    pub fn select_all(&mut self) {
        self.selected_range = 0..self.len_chars();
        self.selection_reversed = false;
    }

    pub fn select_to(&mut self, offset: usize) {
        let offset = self.clamp_offset(offset);

        if self.selection_reversed {
            self.selected_range.start = offset;
        } else {
            self.selected_range.end = offset;
        }

        if self.selected_range.end < self.selected_range.start {
            self.selection_reversed = !self.selection_reversed;
            self.selected_range = self.selected_range.end..self.selected_range.start;
        }
    }

    pub fn backspace(&mut self) {
        if self.has_selection() {
            self.delete_selection();
        } else {
            let cursor = self.cursor_offset();
            if cursor > 0 {
                let prev = self.previous_boundary(cursor);
                self.replace_range(prev..cursor, "");
            }
        }
    }

    pub fn delete(&mut self) {
        if self.has_selection() {
            self.delete_selection();
        } else {
            let cursor = self.cursor_offset();
            if cursor < self.len_chars() {
                let next = self.next_boundary(cursor);
                self.replace_range(cursor..next, "");
            }
        }
    }

    pub fn insert_text(&mut self, text: &str) {
        self.replace_range(self.selected_range.clone(), text);
    }

    pub fn replace_range(&mut self, range: Range<usize>, text: &str) {
        let range = self.clamp_range(range);
        let sanitized_text = Self::sanitize_content(text, self.mode);

        self.content.remove(range.clone());
        self.content.insert(range.start, &sanitized_text);

        let new_cursor = range.start + sanitized_text.chars().count();
        self.selected_range = new_cursor..new_cursor;
        self.selection_reversed = false;
        self.marked_range = None;
    }

    fn delete_selection(&mut self) {
        if self.has_selection() {
            self.replace_range(self.selected_range.clone(), "");
        }
    }

    // UTF-16 conversion for platform APIs
    // Note: GPUI uses byte-based indexing, so we need to convert char offsets to byte offsets

    pub fn char_to_byte(&self, char_offset: usize) -> usize {
        self.content.char_to_byte(char_offset.min(self.len_chars()))
    }

    pub fn byte_to_char(&self, byte_offset: usize) -> usize {
        self.content
            .byte_to_char(byte_offset.min(self.content.len_bytes()))
    }

    pub fn offset_from_utf16(&self, utf16_offset: usize) -> usize {
        let mut char_offset = 0;
        let mut utf16_count = 0;

        for character in self.content.chars() {
            if utf16_count >= utf16_offset {
                break;
            }
            utf16_count += character.len_utf16();
            char_offset += 1;
        }

        char_offset
    }

    pub fn offset_to_utf16(&self, char_offset: usize) -> usize {
        let mut utf16_offset = 0;

        for character in self.content.chars().take(char_offset) {
            utf16_offset += character.len_utf16();
        }

        utf16_offset
    }

    pub fn range_from_utf16(&self, range: Range<usize>) -> Range<usize> {
        self.offset_from_utf16(range.start)..self.offset_from_utf16(range.end)
    }

    pub fn range_to_utf16(&self, range: Range<usize>) -> Range<usize> {
        self.offset_to_utf16(range.start)..self.offset_to_utf16(range.end)
    }

    pub fn text_for_range(&self, range: Range<usize>) -> String {
        let clamped = self.clamp_range(range);
        self.content.slice(clamped).to_string()
    }

    fn previous_boundary(&self, offset: usize) -> usize {
        if offset == 0 {
            return 0;
        }

        let start = offset.saturating_sub(10).max(0);
        let end = offset;
        let slice = self.content.slice(start..end).to_string();

        slice
            .grapheme_indices(true)
            .rev()
            .nth(0)
            .map(|(idx, _)| start + idx)
            .unwrap_or(start)
    }

    fn next_boundary(&self, offset: usize) -> usize {
        if offset >= self.len_chars() {
            return self.len_chars();
        }

        let start = offset;
        let end = (offset + 10).min(self.len_chars());
        let slice = self.content.slice(start..end).to_string();

        slice
            .grapheme_indices(true)
            .nth(1)
            .map(|(idx, _)| start + idx)
            .unwrap_or(end)
    }

    fn line_start_offset(&self, offset: usize) -> usize {
        if self.mode == BufferMode::SingleLine {
            return 0;
        }

        let line_idx = self.content.char_to_line(offset);
        self.content.line_to_char(line_idx)
    }

    fn line_end_offset(&self, offset: usize) -> usize {
        if self.mode == BufferMode::SingleLine {
            return self.len_chars();
        }

        let line_idx = self.content.char_to_line(offset);
        let line_start = self.content.line_to_char(line_idx);
        let line = self.content.line(line_idx);
        let line_len = line.len_chars();

        // Don't include the newline character
        let line_content_len = if line_len > 0 && line.char(line_len - 1) == '\n' {
            line_len - 1
        } else {
            line_len
        };

        line_start + line_content_len
    }

    fn offset_one_line_up(&self, offset: usize) -> Option<usize> {
        if self.mode == BufferMode::SingleLine {
            return None;
        }

        let current_line = self.content.char_to_line(offset);
        if current_line == 0 {
            return None;
        }

        let current_line_start = self.content.line_to_char(current_line);
        let column = offset - current_line_start;

        let prev_line = current_line - 1;
        let prev_line_start = self.content.line_to_char(prev_line);
        let prev_line_content = self.content.line(prev_line);
        let prev_line_len = prev_line_content.len_chars();

        // Don't count the newline
        let prev_line_content_len =
            if prev_line_len > 0 && prev_line_content.char(prev_line_len - 1) == '\n' {
                prev_line_len - 1
            } else {
                prev_line_len
            };

        Some(prev_line_start + column.min(prev_line_content_len))
    }

    fn offset_one_line_down(&self, offset: usize) -> Option<usize> {
        if self.mode == BufferMode::SingleLine {
            return None;
        }

        let current_line = self.content.char_to_line(offset);
        if current_line >= self.len_lines() - 1 {
            return None;
        }

        let current_line_start = self.content.line_to_char(current_line);
        let column = offset - current_line_start;

        let next_line = current_line + 1;
        let next_line_start = self.content.line_to_char(next_line);
        let next_line_content = self.content.line(next_line);
        let next_line_len = next_line_content.len_chars();

        // Don't count the newline
        let next_line_content_len =
            if next_line_len > 0 && next_line_content.char(next_line_len - 1) == '\n' {
                next_line_len - 1
            } else {
                next_line_len
            };

        Some(next_line_start + column.min(next_line_content_len))
    }

    fn clamp_offset(&self, offset: usize) -> usize {
        offset.min(self.len_chars())
    }

    fn clamp_range(&self, range: Range<usize>) -> Range<usize> {
        let len = self.len_chars();
        let start = range.start.min(len);
        let end = range.end.min(len);
        start..end
    }

    fn sanitize_content(content: &str, mode: BufferMode) -> String {
        match mode {
            BufferMode::SingleLine => content.replace('\n', " "),
            BufferMode::MultiLine => content.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_buffer() {
        let buffer = InputBuffer::new("hello");
        assert_eq!(buffer.content(), "hello");
        assert_eq!(buffer.cursor_offset(), 0);
        assert!(!buffer.has_selection());
    }

    #[test]
    fn test_single_line_mode_sanitizes_newlines() {
        let buffer = InputBuffer::single_line("hello\nworld");
        assert_eq!(buffer.content(), "hello world");
    }

    #[test]
    fn test_multiline_mode_preserves_newlines() {
        let buffer = InputBuffer::new("hello\nworld");
        assert_eq!(buffer.content(), "hello\nworld");
    }

    #[test]
    fn test_move_cursor() {
        let mut buffer = InputBuffer::new("hello");
        buffer.move_to_end();
        assert_eq!(buffer.cursor_offset(), 5);

        buffer.move_to_start();
        assert_eq!(buffer.cursor_offset(), 0);

        buffer.move_to(3);
        assert_eq!(buffer.cursor_offset(), 3);
    }

    #[test]
    fn test_movement() {
        let mut buffer = InputBuffer::new("hello");
        buffer.move_to(0);

        buffer.move_right();
        assert_eq!(buffer.cursor_offset(), 1);

        buffer.move_left();
        assert_eq!(buffer.cursor_offset(), 0);
    }

    #[test]
    fn test_selection() {
        let mut buffer = InputBuffer::new("hello");
        buffer.move_to(1);
        buffer.select_to(4);

        assert!(buffer.has_selection());
        assert_eq!(buffer.selected_text(), "ell");
    }

    #[test]
    fn test_select_all() {
        let mut buffer = InputBuffer::new("hello");
        buffer.select_all();

        assert_eq!(buffer.selected_text(), "hello");
    }

    #[test]
    fn test_insert_text() {
        let mut buffer = InputBuffer::new("hello");
        buffer.move_to(5);
        buffer.insert_text(" world");

        assert_eq!(buffer.content(), "hello world");
        assert_eq!(buffer.cursor_offset(), 11);
    }

    #[test]
    fn test_insert_replaces_selection() {
        let mut buffer = InputBuffer::new("hello world");
        buffer.select_all();
        buffer.insert_text("hi");

        assert_eq!(buffer.content(), "hi");
        assert_eq!(buffer.cursor_offset(), 2);
    }

    #[test]
    fn test_backspace() {
        let mut buffer = InputBuffer::new("hello");
        buffer.move_to_end();
        buffer.backspace();

        assert_eq!(buffer.content(), "hell");
        assert_eq!(buffer.cursor_offset(), 4);
    }

    #[test]
    fn test_delete() {
        let mut buffer = InputBuffer::new("hello");
        buffer.move_to(0);
        buffer.delete();

        assert_eq!(buffer.content(), "ello");
        assert_eq!(buffer.cursor_offset(), 0);
    }

    #[test]
    fn test_multiline_navigation() {
        let mut buffer = InputBuffer::new("hello\nworld\nfoo");
        buffer.move_to(0);

        buffer.move_down();
        assert_eq!(buffer.cursor_offset(), 6);

        buffer.move_down();
        assert_eq!(buffer.cursor_offset(), 12);

        buffer.move_up();
        assert_eq!(buffer.cursor_offset(), 6);
    }

    #[test]
    fn test_line_start_end() {
        let mut buffer = InputBuffer::new("hello\nworld");
        buffer.move_to(8);

        buffer.move_to_line_start();
        assert_eq!(buffer.cursor_offset(), 6);

        buffer.move_to_line_end();
        assert_eq!(buffer.cursor_offset(), 11);
    }

    #[test]
    fn test_single_line_mode_up_down() {
        let mut buffer = InputBuffer::single_line("hello");
        buffer.move_to(2);

        buffer.move_up();
        assert_eq!(buffer.cursor_offset(), 0);

        buffer.move_down();
        assert_eq!(buffer.cursor_offset(), 5);
    }

    #[test]
    fn test_utf16_conversion() {
        let buffer = InputBuffer::new("hello 🌍");
        let char_offset = 6;
        let utf16_offset = buffer.offset_to_utf16(char_offset);
        let back_to_char = buffer.offset_from_utf16(utf16_offset);

        assert_eq!(char_offset, back_to_char);
    }

    #[test]
    fn test_char_byte_conversion() {
        let buffer = InputBuffer::new("hello 🌍");

        // "hello " is 6 chars, 6 bytes
        // 🌍 is 1 char, 4 bytes
        assert_eq!(buffer.char_to_byte(6), 6);
        assert_eq!(buffer.char_to_byte(7), 10);

        assert_eq!(buffer.byte_to_char(6), 6);
        assert_eq!(buffer.byte_to_char(10), 7);
    }

    #[test]
    fn test_len_lines() {
        let buffer = InputBuffer::new("hello\nworld\nfoo");
        assert_eq!(buffer.len_lines(), 3);

        let single = InputBuffer::single_line("hello");
        assert_eq!(single.len_lines(), 1);
    }
}
