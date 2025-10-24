use gpui::{Entity, Focusable, Render};

pub trait Item: Focusable + Render + Sized {}

pub trait ItemHandle: Send + Sync {}

impl<T> ItemHandle for Entity<T> where T: Item {}
