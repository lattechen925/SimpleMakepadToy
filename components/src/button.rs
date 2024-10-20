use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

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
        THEME_COLOR_W = #FFFFFFFF
        THEME_COLOR_W_H = #FFFFFF00

        THEME_COLOR_B = #006FEEFF
        THEME_COLOR_B_H = #006FEE00
// WIDGET COLORS
    THEME_COLOR_CTRL_DEFAULT = (THEME_COLOR_U_1)
    THEME_COLOR_CTRL_PRESSED = (THEME_COLOR_D_1)
    THEME_COLOR_CTRL_HOVER = (THEME_COLOR_U_2)
    THEME_COLOR_CTRL_ACTIVE = (THEME_COLOR_D_2)
    THEME_COLOR_CTRL_SELECTED = (THEME_COLOR_U_2)
    THEME_COLOR_CTRL_INACTIVE = (THEME_COLOR_D_HIDDEN)

        THEME_COLOR_D_1 = (mix(THEME_COLOR_B, THEME_COLOR_B_H, pow(0.85, 1.0)))
        THEME_COLOR_U_2 = (mix(THEME_COLOR_W, THEME_COLOR_W_H, pow(0.9, 1.0)))

        draw_bg: {
            instance hover: 0.0
            instance pressed: 0.0
            uniform border_radius: 8.0
            instance bodytop: (THEME_COLOR_D_1)
            instance bodybottom: (THEME_COLOR_D_2)

            fn get_color(self) -> vec4 {
                let a = #006FEE;
                let b = vec4(a.rgb, 0.8)

                return mix(a, b, self.hover)

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
                sdf.fill_keep(self.get_color())
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

// #[derive(Live, LiveHook)]
// #[live_ignore]
pub enum SiButtonColor {
    // #[pick]
    None,
    Primary(String),
    Secondary,
    Success,
    Warning,
    Danger,
}

impl Default for SiButtonColor {
    fn default() -> Self {
        SiButtonColor::None
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct SiButton {
    #[deref]
    button: Button,

    // #[live]color: SiButtonColor,
}

impl Widget for SiButton {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.button.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.button.draw_walk(cx, scope, walk)
    }
}

impl SiButton {

    /// Returns `true` if this button was clicked.
    ///
    /// See [`ButtonAction`] for more details.
    pub fn clicked(&self, actions: &Actions) -> bool {
        self.clicked_modifiers(actions).is_some()
    }

    /// Returns `true` if this button was pressed down.
    ///
    /// See [`ButtonAction`] for more details.
    pub fn pressed(&self, actions: &Actions) -> bool {
        self.pressed_modifiers(actions).is_some()
    }

    /// Returns `true` if this button was released, which is *not* considered to be clicked.
    ///
    /// See [`ButtonAction`] for more details.
    pub fn released(&self, actions: &Actions) -> bool {
        self.released_modifiers(actions).is_some()
    }

    /// Returns `Some` (with active keyboard modifiers) if this button was clicked.
    ///
    /// See [`ButtonAction`] for more details.
    pub fn clicked_modifiers(&self, actions: &Actions) -> Option<KeyModifiers> {
        if let ButtonAction::Clicked(m) = actions.find_widget_action(self.widget_uid()).cast_ref() {
            Some(*m)
        } else {
            None
        }
    }

    /// Returns `Some` (with active keyboard modifiers) if this button was pressed down.
    ///
    /// See [`ButtonAction`] for more details.
    pub fn pressed_modifiers(&self, actions: &Actions) -> Option<KeyModifiers> {
        if let ButtonAction::Pressed(m) = actions.find_widget_action(self.widget_uid()).cast_ref() {
            Some(*m)
        } else {
            None
        }
    }

    /// Returns `Some` (with active keyboard modifiers) if this button was released,
    /// which is *not* considered to be clicked.
    ///
    /// See [`ButtonAction`] for more details.
    pub fn released_modifiers(&self, actions: &Actions) -> Option<KeyModifiers> {
        if let ButtonAction::Released(m) = actions.find_widget_action(self.widget_uid()).cast_ref() {
            Some(*m)
        } else {
            None
        }
    }
}

impl SiButtonRef {
    /// See [`Button::clicked()`].
    pub fn clicked(&self, actions: &Actions) -> bool {
        self.borrow().map_or(false, |inner| inner.clicked(actions))
    }

    /// See [`Button::pressed()`].
    pub fn pressed(&self, actions: &Actions) -> bool {
        self.borrow().map_or(false, |inner| inner.pressed(actions))
    }

    /// See [`Button::released()`].
    pub fn released(&self, actions: &Actions) -> bool {
        self.borrow().map_or(false, |inner| inner.released(actions))
    }

    /// See [`Button::clicked_modifiers()`].
    pub fn clicked_modifiers(&self, actions: &Actions) -> Option<KeyModifiers> {
        self.borrow().and_then(|inner| inner.clicked_modifiers(actions))
    }

    /// See [`Button::pressed_modifiers()`].
    pub fn pressed_modifiers(&self, actions: &Actions) ->  Option<KeyModifiers> {
        self.borrow().and_then(|inner| inner.pressed_modifiers(actions))
    }

    /// See [`Button::released_modifiers()`].
    pub fn released_modifiers(&self, actions: &Actions) -> Option<KeyModifiers> {
        self.borrow().and_then(|inner| inner.released_modifiers(actions))
    }

    // pub fn set_visible(&self, visible: bool) {
    //     if let Some(mut inner) = self.borrow_mut() {
    //         inner.visible = visible;
    //     }
    // }
    //
    // pub fn set_enabled(&self, enabled: bool) {
    //     if let Some(mut inner) = self.borrow_mut() {
    //         inner.enabled = enabled;
    //     }
    // }

    /// Resets the hover state of this button. This is useful in certain cases the
    /// hover state should be reseted in a specific way that is not the default behavior
    /// which is based on the mouse cursor position and movement.
    pub fn reset_hover(&self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.animator_cut(cx, id!(hover.off));
        }
    }
}