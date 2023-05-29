use std::{
    fmt::{self, Display},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use async_std::{channel, task::JoinHandle};
use osd_core::{ClientMessage, DeviceInfo, ServerMessage, SERVER_IP, SERVER_PORT};

pub struct StatusEvent {
    pub message: String,
}

pub fn advertise(
    info: &DeviceInfo,
    notice_sender: nwg::NoticeSender,
) -> (channel::Receiver<StatusEvent>, JoinHandle<()>) {
    let (tx, rx) = channel::unbounded();

    let handle = async_std::spawn(move || {
        tx.send(StatusEvent {
            message: "Attempting to connect to the server".to_string(),
        })
        .await
        .unwrap();
        notice_sender.notice();

        // Attemt to connect with initial timeout of 10. Retry infinitely.
        let mut stream =
            TcpStream::connect_timeout((SERVER_IP, SERVER_PORT).into(), Duration::from_secs(10));

        let mut try_count = 0u32;

        while let Err(_) = stream {
            try_count += 1;
            tx.send(StatusEvent {
                message: format!("Can't connect to server, retrying in 5s. Try #{try_count}"),
            })
            .await
            .unwrap();
            notice_sender.notice();

            thread::sleep(Duration::from_secs(5));
            stream = TcpStream::connect((SERVER_IP, SERVER_PORT));
        }

        let Ok(mut stream) = stream else { unreachable!() };

        let document = bson::to_document(&ClientMessage::Advertise(info.clone())).unwrap();

        document.to_writer(&mut stream).unwrap();

        // TODO:
        //  - Send advertisement to server
        //  - Wait for rebuild command

        tx.send(StatusEvent {
            message: "Waiting for rebuild command".to_string(),
        })
        .await
        .unwrap();

        notice_sender.notice();
    });

    (rx, handle)
}
