use smt_components::*;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import smt_components::base::*;

    App = {{App}} {
        ui: <Root> {
            main_window = <Window> {
                window: {title: "SimpleMakepadToy"},
                caption_bar = { caption_label = {label = {text: "SimpleMakepadToy"}} },
                // show_bg: true
                // draw_bg: {
                //     fn pixel(self) -> vec4{
                //         return #ffffff;
                //     }
                // }

                body = <ScrollXYView>{
                    flow: Down,
                    spacing:10,
                    align: {
                        x: 0.5,
                        y: 0.5
                    },
                    show_bg: true
                    draw_bg: {
                        fn pixel(self) -> vec4{
                            return #f;
                        }
                    }

                    myBtn = <SiButtonBase> { text: "Default(Medium)" }
                    myBtn2 = <SiButtonBase> { text: "Primary Full", color: Primary, radius: full }
                    myBtn3 = <SiButtonBase> { text: "Secondary Large", color: Secondary, radius: lg }
                    myBtn4 = <SiButtonBase> { text: "Success Medium", color: Success, radius: md }
                    myBtn5 = <SiButtonBase> { text: "Warning Small", color: Warning, radius: sm }
                    myBtn6 = <SiButtonBase> { text: "Danger None", color: Danger, radius: none }

                    button = <Button> { text: "Button" }
                }
            }
        }
    }
}

app_main!(App);

#[derive(Live, LiveHook)]
pub struct App {
    #[live]ui: WidgetRef,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        smt_components::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, _cx: &mut Cx, actions: &Actions) {
        //
        if self.ui.si_button(id!(myBtn)).clicked(&actions) {
            println!("button clicked: SiButtonBase");
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}