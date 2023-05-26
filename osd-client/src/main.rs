#[cfg(target_os = "windows")]
mod device_windows;
#[cfg(target_os = "windows")]
use device_windows as device;

#[cfg(not(target_os = "windows"))]
mod device_other;
#[cfg(not(target_os = "windows"))]
use device_other as device;

use nwd::NwgUi;
use nwg::NativeUi;
use osd_core::Mac;

fn main() {
    let is_wine = std::env::vars().any(|(key, _)| key == "WINELOADER");
    dbg!(is_wine);

    nwg::init().expect("Failed to initialize Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let maybe_mac_address = device::get_local_mac_address();

    let mac_address = if is_wine {
        maybe_mac_address.unwrap_or_else(|_| Mac("A0:B1:C2:D3:E4:F5".to_owned()))
    } else {
        maybe_mac_address.expect("Failed to get local MAC address")
    };

    let init = OsdClientApp {
        mac_address,
        hostname: None,
        ..Default::default()
    };
    let _app = OsdClientApp::build_ui(init).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}

const LABEL_X: i32 = 15;
const LABEL_W: i32 = 110;
const LABEL_H: i32 = 24;
const LABEL_SIZE: (i32, i32) = (LABEL_W, LABEL_H);

const TEXT_INPUT_X: i32 = 132;
const TEXT_INPUT_W: i32 = 152;
const TEXT_INPUT_H: i32 = 24;
const TEXT_INPUT_SIZE: (i32, i32) = (TEXT_INPUT_W, TEXT_INPUT_H);

const ROW_A_Y: i32 = 20;
const ROW_B_Y: i32 = 54;

#[derive(Default, NwgUi)]
pub struct OsdClientApp {
    mac_address: Mac,
    hostname: Option<String>,

    #[nwg_control(size: (300, 130), title: "Mass OS Deployment Advertiser")]
    #[nwg_events( 
        OnInit: [OsdClientApp::on_init],
        OnWindowClose: [OsdClientApp::on_close] 
    )]
    window: nwg::Window,

    #[nwg_control(text: "MAC Address:", position: (LABEL_X, ROW_A_Y), size: LABEL_SIZE)]
    l1: nwg::Label,

    #[nwg_control(text: "Hostname:", position: (LABEL_X, ROW_B_Y), size: LABEL_SIZE)]
    l2: nwg::Label,

    #[nwg_control(
        text: "", 
        readonly: true,
        position: (TEXT_INPUT_X, ROW_A_Y),
        size: TEXT_INPUT_SIZE
    )]
    mac_text: nwg::TextInput,

    #[nwg_control(
        text: "", 
        readonly: true,
        position: (TEXT_INPUT_X, ROW_B_Y),
        size: TEXT_INPUT_SIZE
    )]
    hostname_text: nwg::TextInput,

    #[nwg_control(text: "Exit Now", size: (100, 30), position: (190, 95))]
    #[nwg_events( OnButtonClick: [OsdClientApp::quit_button_pressed] )]
    quit: nwg::Button,
}

impl OsdClientApp {
    fn on_init(&self) {
        self.mac_text.set_text(&self.mac_address.0);
        self.set_hostname(self.hostname.clone());
    }

    fn set_hostname(&self, hostname: Option<String>) {
        match hostname {
            Some(name) => self.hostname_text.set_text(&name),
            None => {
                self.hostname_text.set_text("Not Found");
                self.hostname_text.set_enabled(false);
            }
        }
    }

    fn on_close(&self) {
        nwg::stop_thread_dispatch();
    }

    fn quit_button_pressed(&self) {
        println!("Quit button pressed!");
    }
}
