use std::collections::BTreeMap;
use makepad_widgets::*;
use makepad_derive_live::*;
use makepad_widgets::makepad_live_compiler::LiveScopeTarget;
use crate::{u32_to_vec4};

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
            instance color: #f,
            text_style: <THEME_FONT_REGULAR> {
                font_size: 12.0
            }
            fn get_color(self) -> vec4 {
                return self.color;
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
            uniform border_radius: (SI_RADIUS_MEDIUM)
            instance background_color: (SI_DEFAULT)

            fn get_color(self, color: vec4) -> vec4 {
                // let body = mix(mix(self.bodytop, self.bodybottom, self.hover), THEME_COLOR_CTRL_PRESSED, self.pressed);
                return mix(mix(color, vec4(color.rgb, SI_HOVER_OPACITY), self.hover), mix(color, #000000, 0.05), self.pressed)
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

#[derive(Live, LiveHook, LiveRegister, Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
#[live_ignore]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SiButtonRadius {
    full = shader_enum(1),
    lg = shader_enum(2),
    #[pick]
    md = shader_enum(3),
    sm = shader_enum(4),
    none = shader_enum(5),
}

#[derive(Live, LiveHook, LiveRegister)]
#[live_ignore]
#[repr(u32)]

pub enum SiButtonVariant {
    #[pick]
    Solid = shader_enum(1),
    Faded = shader_enum(2),
    Bordered = shader_enum(3),
    Light = shader_enum(4),
    Flat = shader_enum(5),
    Ghost = shader_enum(6),
    Shadow = shader_enum(7),
}

#[derive(Live, LiveHook, LiveRegister)]
#[live_ignore]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum SiButtonSize {
    #[pick]
    sm = shader_enum(1),
    md = shader_enum(2),
    lg = shader_enum(3),
}

#[derive(Live, Widget)]
pub struct SiButton {
    #[deref]
    button: Button,

    #[live] color: SiButtonColor,
    #[live] radius: SiButtonRadius,
    #[live] variant: SiButtonVariant,

    #[rust] color_map: BTreeMap<SiButtonColor, Vec4>,
    #[rust] radius_map: BTreeMap<SiButtonRadius, f64>,
}

impl LiveHook for SiButton {
    fn after_new_before_apply(&mut self, cx: &mut Cx) {
        // LiveModuleId -> LiveFile -> LiveExpanded
        let module_id = LiveModuleId::from_str(module_path!()).unwrap();
        let live_registry = cx.live_registry.borrow();

        if let Some(nodes) = live_registry.module_id_to_expanded_nodes(module_id) {
            let root = nodes.get(0).unwrap();

            let file_id = if let LiveValue::Root {id_resolve} = &root.value {
                let live_scope_target = id_resolve.get(&live_id!(SI_DEFAULT)).unwrap();
                if let LiveScopeTarget::LivePtr(live_ptr) = live_scope_target {
                    Some(live_ptr.file_id)
                } else {
                    None
                }
            } else {
                None
            };

            if let Some(file_id) = file_id {
                let file = live_registry.file_id_to_file(file_id);
                let expanded_nodes = &file.expanded.nodes;

                expanded_nodes.iter().for_each(|node| {
                    if node.is_instance_prop() {
                        self.insert_color(node, live_id!(SI_DEFAULT), SiButtonColor::Default);
                        self.insert_color(node, live_id!(SI_PRIMARY), SiButtonColor::Primary);
                        self.insert_color(node, live_id!(SI_SECONDARY), SiButtonColor::Secondary);
                        self.insert_color(node, live_id!(SI_SUCCESS), SiButtonColor::Success);
                        self.insert_color(node, live_id!(SI_WARNING), SiButtonColor::Warning);
                        self.insert_color(node, live_id!(SI_DANGER), SiButtonColor::Danger);

                        self.insert_radius(node, live_id!(SI_RADIUS_FULL), SiButtonRadius::full);
                        self.insert_radius(node, live_id!(SI_RADIUS_LARGE), SiButtonRadius::lg);
                        self.insert_radius(node, live_id!(SI_RADIUS_MEDIUM), SiButtonRadius::md);
                        self.insert_radius(node, live_id!(SI_RADIUS_SMALL), SiButtonRadius::sm);
                        self.insert_radius(node, live_id!(SI_RADIUS_NONE), SiButtonRadius::none);
                    }
                })
            }
        }

        println!("--: {:#?}", self.radius_map);
        // let styles_path = get_module_path!("styles".into());
        // let lr = cx.live_registry.borrow();
        // let mut insert_map = |node: &LiveNode, style_color: LiveId, key: SiButtonColor| {
        //     if node.id == style_color {
        //         if let LiveValue::Color(color) = node.value {
        //             self.color_map.insert(key, u32_to_vec4(color));
        //         }
        //     }
        // };
        // if let Some(expanded_nodes) = lr.module_id_to_expanded_nodes(LiveModuleId::from_str(styles_path.as_str()).unwrap()) {
        //     expanded_nodes.iter().for_each(|node| {
        //         if node.is_instance_prop() {
        //             insert_map(node, live_id!(SI_DEFAULT), SiButtonColor::Default);
        //             insert_map(node, live_id!(SI_PRIMARY), SiButtonColor::Primary);
        //             insert_map(node, live_id!(SI_SECONDARY), SiButtonColor::Secondary);
        //             insert_map(node, live_id!(SI_SUCCESS), SiButtonColor::Success);
        //             insert_map(node, live_id!(SI_WARNING), SiButtonColor::Warning);
        //             insert_map(node, live_id!(SI_DANGER), SiButtonColor::Danger);
        //         }
        //     })
        // }

        // println!("map: {:#?}", self.color_map);
        // println!("after_apply end");
    }
}

impl Widget for SiButton {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.button.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if self.color != SiButtonColor::Default {
            let background_color = self.color_map.get(&self.color).unwrap();

            self.apply_over(cx, live! {
                draw_bg: { background_color: (background_color) }
            });
        } else {
            self.apply_over(cx, live! {
                draw_text: { color: (vec3(0.0,0.0,0.0)) }
            });
        }
        if self.radius != SiButtonRadius::md {
            let border_radius = self.radius_map.get(&self.radius).unwrap();

            self.apply_over(cx, live! {
                draw_bg: { border_radius: (border_radius) }
            });
        }
        self.button.draw_walk(cx, scope, walk)
    }
}

impl SiButton {
    fn insert_color(&mut self, node: &LiveNode, style_color: LiveId, key: SiButtonColor) {
        if node.id == style_color {
            if let LiveValue::Color(color) = node.value {
                self.color_map.insert(key, u32_to_vec4(color));
            }
        }
    }

    fn insert_radius(&mut self, node: &LiveNode, style_radius: LiveId, key: SiButtonRadius) {
        if node.id == style_radius {
            // println!("insert_radius: {:#?}", node)
            if let LiveValue::Float64(val) = node.value {
                self.radius_map.insert(key, val);
            }
        }
    }
}

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