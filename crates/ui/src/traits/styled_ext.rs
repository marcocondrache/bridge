use gpui::Styled;

pub trait StyledExt: Styled + Sized {
    fn h_flex(self) -> Self {
        self.flex().flex_row()
    }

    fn v_flex(self) -> Self {
        self.flex().flex_col()
    }
}

impl<E: Styled> StyledExt for E {}
