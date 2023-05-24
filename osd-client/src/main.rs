use nwd::NwgUi;
use nwg::NativeUi;

use nwg::stretch::style::FlexDirection;


fn main() {
    nwg::init().expect("Failed to initialize Native Windows GUI");

    let init = OsdClientApp {
        mac_address: "12:34:56:78:90:AB".to_string(),
        ..Default::default()
    };
    let _app = OsdClientApp::build_ui(init).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}

#[derive(Default, NwgUi)]
pub struct OsdClientApp {
    // data: ,
    mac_address: String,

    #[nwg_control(size: (300, 130), position: (300, 300), title: "Mass OS Deployment Advertiser", flags: "WINDOW|VISIBLE")]
    #[nwg_events( 
        OnInit: [OsdClientApp::on_init],
        OnWindowClose: [OsdClientApp::on_close] 
    )]
    window: nwg::Window,

    // #[nwg_layout(flex_direction: FlexDirection::Row)]
    // flex: nwg::FlexboxLayout,

    #[nwg_layout(parent: window, spacing: 2, margin: [5, 5, 5, 5])]
    grid: nwg::GridLayout,

    #[nwg_control(text: "MAC Address:")]
    #[nwg_layout_item(layout: grid, col: 0, row: 0)]
    l1: nwg::Label,

    #[nwg_control(text: "Hostname:")]
    #[nwg_layout_item(layout: grid, col: 0, row: 1)]
    l2: nwg::Label,

    #[nwg_control(text: "", readonly: true)]
    #[nwg_layout_item(layout: grid, col: 1, row: 0)]
    mac_text: nwg::TextInput,

    #[nwg_control(text: "", readonly: true)]
    #[nwg_layout_item(layout: grid, col: 1, row: 1)]
    hostname_text: nwg::TextInput,

    #[nwg_control(text: "Exit Now", size: (100, 40))]
    #[nwg_layout_item(layout: grid, col: 1, row: 2)]
    quit: nwg::Button,
}

impl OsdClientApp {
    fn on_init(&self) {
        self.mac_text.set_text(&self.mac_address);
    }

    fn on_close(&self) {
        // nwg::simple_message("Goodbye", "Goodbye, friend!");
        nwg::stop_thread_dispatch();
    }
}
