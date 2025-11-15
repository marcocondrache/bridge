pub mod components;
pub mod traits;
pub mod utils;
pub mod variants;

pub mod prelude {
    pub use crate::variants::{Layout, Semantic, Size, Spacing};

    pub use crate::traits::{
        clickable::Clickable, disableable::Disableable, layoutable::Layoutable, loadable::Loadable,
        semantic_color::SemanticColor, sizable::Sizable, styled_ext::StyledExt,
        validatable::Validatable,
    };
}
