use std::{
    fmt::{self, Display},
    net::{TcpListener, TcpStream},
    sync::mpsc,
    thread,
    time::Duration,
};

use osd_core::{DeviceInfo, ServerMessage, SERVER_IP, SERVER_PORT};

pub struct StatusEvent {
    pub message: String,
}

pub fn advertise(
    info: &DeviceInfo,
    notice_sender: nwg::NoticeSender,
    close_rx: mpsc::Receiver<()>,
) -> mpsc::Receiver<StatusEvent> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(StatusEvent {
            message: "Attempting to connect to the server".to_string(),
        })
        .unwrap();
        notice_sender.notice();

        // Attemt to connect with initial timeout of 10. Retry infinitely.
        let mut stream =
            TcpStream::connect_timeout((SERVER_IP, SERVER_PORT), Duration::from_secs(10));

        let mut try_count = 0u32;

        while let Err(_) = stream {
            try_count += 1;
            tx.send(StatusEvent {
                message: format!("Can't connect to server, retrying in 5s. Try #{try_count}"),
            })
            .unwrap();
            notice_sender.notice();

            thread::sleep(Duration::from_secs(5));
            stream = TcpStream::connect((SERVER_IP, SERVER_PORT));
        }

        // TODO:
        //  - Send advertisement to server
        //  - Wait for rebuild command

        tx.send(StatusEvent {
            message: "Waiting for rebuild command".to_string(),
        })
        .unwrap();
        notice_sender.notice();
    });

    rx
}
