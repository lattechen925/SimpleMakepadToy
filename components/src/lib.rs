use makepad_widgets::Cx;
pub use makepad_widgets;

pub mod button;
pub mod base;

pub use crate::{
    button::*
};

pub fn live_design(cx: &mut Cx) {
    crate::button::live_design(cx);
    crate::base::live_design(cx);
}