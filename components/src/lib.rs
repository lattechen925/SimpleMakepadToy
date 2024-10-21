use makepad_widgets::{vec4, Cx, Vec4};
pub use makepad_widgets;

pub mod button;
pub mod base;
pub mod styles;

pub use crate::button::*;

pub fn live_design(cx: &mut Cx) {
    crate::styles::live_design(cx);
    crate::button::live_design(cx);
    crate::base::live_design(cx);
}

fn u32_to_vec4(value: u32) -> Vec4 {
    let r = ((value >> 24) & 0xFF) as f32 / 255.0;
    let g = ((value >> 16) & 0xFF) as f32 / 255.0;
    let b = ((value >> 8) & 0xFF) as f32 / 255.0;
    let a = (value & 0xFF) as f32 / 255.0;
    vec4(r, g, b, a)
}

#[macro_export]
macro_rules! get_module_path {
    ( $( $x:expr ),* ) => {
        {
            let mut p:Vec<String> = Vec::new();
            p.push(env!("CARGO_PKG_NAME").replace("-", "_"));
            $(
                p.push($x);
            )*
            p.join("::")
        }
    };
}

// pub use get_module_path;