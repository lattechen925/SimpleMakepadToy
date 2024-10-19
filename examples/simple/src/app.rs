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

                body = <ScrollXYView>{
                    flow: Down,
                    spacing:10,
                    align: {
                        x: 0.5,
                        y: 0.5
                    },

                    myBtn = <SiButtonBase> { text: "My SiButtonBase" }
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