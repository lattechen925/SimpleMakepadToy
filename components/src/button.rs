use std::collections::BTreeMap;
use makepad_widgets::*;
use makepad_derive_live::*;
use crate::{get_module_path, u32_to_vec4};

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;
    import crate::styles::*;


    SiButtonBase = {{SiButton}}{
        // TODO: NEEDS FOCUS STATE

        width: Fit, height: Fit,
        spacing: 7.5,
        align: {x: 0.5, y: 0.5},
        padding: {top: 10.0, right: 16.0, bottom: 10.0, left: 16.0}
        label_walk: { width: Fit, height: Fit },

        draw_text: {
            instance hover: 0.0,
            instance pressed: 0.0,
            text_style: <THEME_FONT_REGULAR> {
                font_size: 14.0
            }
            fn get_color(self) -> vec4 {
                return #f;
            }
        }

        // icon_walk: {
        //     width: (THEME_DATA_ICON_WIDTH), height: Fit,
        // }
        //
        // draw_icon: {
        //     instance hover: 0.0
        //     instance pressed: 0.0
        //     uniform color: (THEME_COLOR_TEXT_DEFAULT)
        //     fn get_color(self) -> vec4 {
        //         return mix(
        //             mix(
        //                 self.color,
        //                 mix(self.color, #f, 0.5),
        //                 self.hover
        //             ),
        //             self.color * 0.75,
        //             self.pressed
        //         )
        //     }
        // }

        draw_bg: {
            instance hover: 0.0
            instance pressed: 0.0
            uniform border_radius: 8.0

            instance background_color: (SI_DEFAULT)

            fn get_color(self, color: vec4) -> vec4 {
                return mix(color, vec4(color.rgb, SI_HOVER_OPACITY), self.hover)
            }

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                sdf.box(
                    1.,
                    1.,
                    self.rect_size.x - 2.0,
                    self.rect_size.y - 2.0,
                    self.border_radius
                )
                sdf.fill_keep(self.get_color(self.background_color))
                return sdf.result
            }
        }

        animator: {
            hover = {
                default: off,
                off = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_bg: {pressed: 0.0, hover: 0.0}
                        draw_icon: {pressed: 0.0, hover: 0.0}
                        draw_text: {pressed: 0.0, hover: 0.0}
                    }
                }

                on = {
                    from: {
                        all: Forward {duration: 0.1}
                        pressed: Forward {duration: 0.01}
                    }
                    apply: {
                        draw_bg: {pressed: 0.0, hover: [{time: 0.0, value: 1.0}],}
                        draw_icon: {pressed: 0.0, hover: [{time: 0.0, value: 1.0}],}
                        draw_text: {pressed: 0.0, hover: [{time: 0.0, value: 1.0}],}
                    }
                }

                pressed = {
                    from: {all: Forward {duration: 0.2}}
                    apply: {
                        draw_bg: {pressed: [{time: 0.0, value: 1.0}], hover: 1.0,}
                        draw_icon: {pressed: [{time: 0.0, value: 1.0}], hover: 1.0,}
                        draw_text: {pressed: [{time: 0.0, value: 1.0}], hover: 1.0,}
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, LiveRegister, Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
#[live_ignore]
#[repr(u32)]
pub enum SiButtonColor {
    #[pick]
    Default = shader_enum(1),
    Primary = shader_enum(2),
    Secondary = shader_enum(3),
    Success = shader_enum(4),
    Warning = shader_enum(5),
    Danger = shader_enum(6),
}

#[derive(Live, Widget)]
pub struct SiButton {
    #[deref]
    button: Button,

    #[live] background_color: SiButtonColor,

    #[rust] color_map: BTreeMap<SiButtonColor, Vec4>,
}

impl LiveHook for SiButton {
    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        let styles_path = get_module_path!("styles".into());
        let lr = cx.live_registry.borrow();
        let mut insert_map = |node: &LiveNode, style_color: LiveId, key: SiButtonColor| {
            if node.id == style_color {
                if let LiveValue::Color(color) = node.value {
                    self.color_map.insert(key, u32_to_vec4(color));
                }
            }
        };
        if let Some(expanded_nodes) = lr.module_id_to_expanded_nodes(LiveModuleId::from_str(styles_path.as_str()).unwrap()) {
            expanded_nodes.iter().for_each(|node| {
                if node.is_instance_prop() {
                    insert_map(node, live_id!(SI_DEFAULT), SiButtonColor::Default);
                    insert_map(node, live_id!(SI_PRIMARY), SiButtonColor::Primary);
                    insert_map(node, live_id!(SI_SECONDARY), SiButtonColor::Secondary);
                    insert_map(node, live_id!(SI_SUCCESS), SiButtonColor::Success);
                    insert_map(node, live_id!(SI_WARNING), SiButtonColor::Warning);
                    insert_map(node, live_id!(SI_DANGER), SiButtonColor::Danger);
                }
            })
        }

        // println!("map: {:#?}", self.color_map);
        // println!("after_apply end");
    }
}

impl Widget for SiButton {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.button.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if self.background_color != SiButtonColor::Default {
            let background_color = self.color_map.get(&self.background_color).unwrap();

            self.apply_over(cx, live! {
                draw_bg: { background_color: (background_color) }
            });
        }
        self.button.draw_walk(cx, scope, walk)
    }
}

// impl SiButton {
//     /// Returns `true` if this button was clicked.
//     ///
//     /// See [`ButtonAction`] for more details.
//     pub fn clicked(&self, actions: &Actions) -> bool {
//         self.clicked_modifiers(actions).is_some()
//     }
//
//     /// Returns `true` if this button was pressed down.
//     ///
//     /// See [`ButtonAction`] for more details.
//     pub fn pressed(&self, actions: &Actions) -> bool {
//         self.pressed_modifiers(actions).is_some()
//     }
//
//     /// Returns `true` if this button was released, which is *not* considered to be clicked.
//     ///
//     /// See [`ButtonAction`] for more details.
//     pub fn released(&self, actions: &Actions) -> bool {
//         self.released_modifiers(actions).is_some()
//     }
//
//     /// Returns `Some` (with active keyboard modifiers) if this button was clicked.
//     ///
//     /// See [`ButtonAction`] for more details.
//     pub fn clicked_modifiers(&self, actions: &Actions) -> Option<KeyModifiers> {
//         if let ButtonAction::Clicked(m) = actions.find_widget_action(self.widget_uid()).cast_ref() {
//             Some(*m)
//         } else {
//             None
//         }
//     }
//
//     /// Returns `Some` (with active keyboard modifiers) if this button was pressed down.
//     ///
//     /// See [`ButtonAction`] for more details.
//     pub fn pressed_modifiers(&self, actions: &Actions) -> Option<KeyModifiers> {
//         if let ButtonAction::Pressed(m) = actions.find_widget_action(self.widget_uid()).cast_ref() {
//             Some(*m)
//         } else {
//             None
//         }
//     }
//
//     /// Returns `Some` (with active keyboard modifiers) if this button was released,
//     /// which is *not* considered to be clicked.
//     ///
//     /// See [`ButtonAction`] for more details.
//     pub fn released_modifiers(&self, actions: &Actions) -> Option<KeyModifiers> {
//         if let ButtonAction::Released(m) = actions.find_widget_action(self.widget_uid()).cast_ref() {
//             Some(*m)
//         } else {
//             None
//         }
//     }
// }

impl SiButtonRef {
    /// See [`Button::clicked()`].
    pub fn clicked(&self, actions: &Actions) -> bool {
        self.borrow().map_or(false, |b| b.clicked(actions))
    }

    /// See [`Button::pressed()`].
    pub fn pressed(&self, actions: &Actions) -> bool {
        self.borrow().map_or(false, |b| b.pressed(actions))
    }

    /// See [`Button::released()`].
    pub fn released(&self, actions: &Actions) -> bool {
        self.borrow().map_or(false, |b| b.released(actions))
    }

    /// See [`Button::clicked_modifiers()`].
    pub fn clicked_modifiers(&self, actions: &Actions) -> Option<KeyModifiers> {
        self.borrow().and_then(|b| b.clicked_modifiers(actions))
    }

    /// See [`Button::pressed_modifiers()`].
    pub fn pressed_modifiers(&self, actions: &Actions) -> Option<KeyModifiers> {
        self.borrow().and_then(|b| b.pressed_modifiers(actions))
    }

    /// See [`Button::released_modifiers()`].
    pub fn released_modifiers(&self, actions: &Actions) -> Option<KeyModifiers> {
        self.borrow().and_then(|b| b.released_modifiers(actions))
    }
}