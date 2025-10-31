use gpui::{Refineable, StyleRefinement, Styled};

#[cfg_attr(debug_assertions, gpui_macros::derive_inspector_reflection)]
pub trait StyledExt: Styled + Sized {
    fn h_flex(self) -> Self {
        self.flex().flex_row().items_center()
    }

    fn v_flex(self) -> Self {
        self.flex().flex_col()
    }
}

impl<E: Styled> StyledExt for E {}
