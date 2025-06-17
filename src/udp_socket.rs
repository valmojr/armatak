use arma_rs::Context;
use lazy_static::lazy_static;
use log::info;
use std::net::UdpSocket;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::cot;

pub enum UdpCommand {
  SendMessage(String, Context),
  Stop,
}

pub struct UdpClient {
  pub(crate) tx: Sender<UdpCommand>,
}

impl UdpClient {
  pub fn start(&self, address: String, rx: Receiver<UdpCommand>, ctx: Context) {
    if let Some(ref client) = *UDP_CLIENT.lock().unwrap() {
      client.stop();
    }

    let udp_address = address.clone();

    thread::spawn(move || {
      let socket = match UdpSocket::bind(udp_address) {
        Ok(s) => s,
        Err(e) => {
          let _ = ctx.callback_data(
            "UDP SOCKET ERROR",
            "Failed to bind UDP socket",
            e.to_string(),
          );
          info!("Failed to bind UDP socket: {}", e);
          return;
        }
      };

      let _ = ctx.callback_data("UDP SOCKET", "EUD Connected", address.clone());

      let mut running = true;
      while running {
        match rx.recv() {
          Ok(UdpCommand::SendMessage(message, context)) => {
            if let Err(e) = socket.send_to(message.as_bytes(), &address) {
              info!("Failed to send UDP message: {}", e);
              let _ = context.callback_data(
                "UDP SOCKET ERROR",
                "Failed to send UDP message",
                e.to_string(),
              );
            }
          }
          Ok(UdpCommand::Stop) => {
            running = false;
            info!("Stopping UDP client.");
          }
          Err(error) => {
            info!("Error receiving command: {}", error.to_string());
          }
        }
      }
    });
  }

  pub fn send_payload(&self, context: Context, payload: String) {
    let tx = self.tx.clone();
    thread::spawn(move || {
      tx.send(UdpCommand::SendMessage(payload, context)).unwrap();
    });
  }

  pub fn stop(&self) {
    let tx = self.tx.clone();
    thread::spawn(move || {
      tx.send(UdpCommand::Stop).unwrap();
    });
  }
}

lazy_static! {
  static ref UDP_CLIENT: Arc<Mutex<Option<UdpClient>>> = Arc::new(Mutex::new(None));
}

pub fn start(ctx: Context, address: String) -> &'static str {
  let (tx, rx): (Sender<UdpCommand>, Receiver<UdpCommand>) = mpsc::channel();

  let client = UdpClient { tx };
  client.start(address, rx, ctx);

  let mut client_guard = UDP_CLIENT.lock().unwrap();
  *client_guard = Some(client);

  "Starting UDP Client"
}

pub fn send_payload(ctx: Context, payload: String) -> &'static str {
  if let Some(ref client) = *UDP_CLIENT.lock().unwrap() {
    client.send_payload(ctx, payload);
  } else {
    let _ = ctx.callback_null("UDP SOCKET ERROR", "UDP Socket is not running");
  }

  "Sending payload to UDP server"
}

pub fn send_gps_cot(ctx: Context, cursor_over_time: cot::gps::ExternalPositionPayload) -> &'static str {
  let payload = cursor_over_time.to_cot().convert_to_xml();
  send_payload(ctx, payload);

  "Sending GPS Cursor Over Time to UDP server"
}

pub fn stop(ctx: Context) -> &'static str {
  if let Some(ref client) = *UDP_CLIENT.lock().unwrap() {
    client.stop();
    let _ = ctx.callback_null("UDP SOCKET", "EUD Disconnected");
  } else {
    let _ = ctx.callback_null("UDP SOCKET ERROR", "UDP Socket is not running");
  }

  "Stopping UDP Client"
}
