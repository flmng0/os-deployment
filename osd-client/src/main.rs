mod advertise;
mod device;

use std::{cell::RefCell, sync::mpsc, thread};

use advertise::{StatusEvent, advertise};

use nwd::NwgUi;
use nwg::NativeUi;
use osd_core::{Mac, ServerMessage, DeviceInfo};

fn main() {
    let is_wine = std::env::vars().any(|(key, _)| key == "WINELOADER");

    nwg::init().expect("Failed to initialize Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let maybe_mac_address = device::get_local_mac_address();

    let mac_address = if is_wine {
        maybe_mac_address.unwrap_or_else(|_| Mac("A0:B1:C2:D3:E4:F5".to_owned()))
    } else {
        maybe_mac_address.expect("Failed to get local MAC address")
    };

    let asset_id = device::generate_asset_id();

    let device_info = DeviceInfo {
        mac_address,
        hostname: asset_id,
    };

    let init = OsdClientApp {
        device_info,
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
const TEXT_INPUT_W: i32 = 202;
const TEXT_INPUT_H: i32 = 24;
const TEXT_INPUT_SIZE: (i32, i32) = (TEXT_INPUT_W, TEXT_INPUT_H);

const ROW_A_Y: i32 = 20;
const ROW_B_Y: i32 = 54;

#[derive(Default, NwgUi)]
pub struct OsdClientApp {
    device_info: DeviceInfo,

    status_event_receiver: RefCell<Option<mpsc::Receiver<StatusEvent>>>,
    close_sender: RefCell<Option<mpsc::Sender<()>>>,

    #[nwg_control(size: (350, 140), title: "Mass OS Deployment Advertiser", flags: "WINDOW|VISIBLE")]
    #[nwg_events( 
        OnInit: [OsdClientApp::on_init],
        OnWindowClose: [OsdClientApp::on_close(SELF, EVT_DATA)] 
    )]
    window: nwg::Window,

    #[nwg_control]
    #[nwg_events( OnNotice: [OsdClientApp::handle_status_event] )]
    status_notice: nwg::Notice,

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

    #[nwg_control(
        text: "Initializing...",
    )]
    status_bar: nwg::StatusBar,

    #[nwg_control(parent: status_bar, flags: "MARQUEE", marquee: true)]
    progress_bar: nwg::ProgressBar,
}

impl OsdClientApp {
    fn on_init(&self) {
        self.mac_text.set_text(&self.device_info.mac_address.0);
        self.set_hostname(self.device_info.hostname.clone());

        let notice_sender = self.status_notice.sender();

        let (close_tx, close_rx) = mpsc::channel();
        *self.close_sender.borrow_mut() = Some(close_tx);

        *self.status_event_receiver.borrow_mut() = Some(advertise(&self.device_info, notice_sender, close_rx));
    }

    fn handle_status_event(&self) {
        let mut receiver_ref = self.status_event_receiver.borrow_mut();

        let id = thread::current().id();

        let Some(receiver) = receiver_ref.as_mut() else { return };

        let Ok(status) = receiver.recv() else { unreachable!() };
        self.set_status(&status.message);
    }

    fn set_status(&self, text: &str) {
        self.status_bar.set_text(0, &format!("{text}..."));
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

    fn on_close(&self, data: &nwg::EventData) {
        let nwg::EventData::OnWindowClose(close_data) = data else { unreachable!() };

        let params = nwg::MessageParams {
            title: "Are you sure?",
            content: r#"The application will automatically continue when selected from the interface.

Rebuilding will not work correctly unless this device has been properly added to Configuration Manager.

Are you sure you want to exit?
            "#,
            buttons: nwg::MessageButtons::YesNo,
            icons: nwg::MessageIcons::Question,
        };

        let response = nwg::modal_message(&self.window, &params);

        let should_close = matches!(response, nwg::MessageChoice::Yes);

        close_data.close(should_close);

        if should_close {
            nwg::stop_thread_dispatch();
        }
    }
}
