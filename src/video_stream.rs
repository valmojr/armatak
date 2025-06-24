use arma_rs::Context;
use lazy_static::lazy_static;
use std::io::Write;
use std::process::{Command, Stdio};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

lazy_static! {
    static ref STREAM_CTRL: Mutex<Option<Sender<()>>> = Mutex::new(None);
}

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

fn build_rtsp_url(address: &str, port: &str, stream_path: &str, username: &str, password: &str) -> String {
    if username.is_empty() || password.is_empty() {
        format!("rtsp://{}:{}/{}", address, port, stream_path)
    } else {
        format!(
            "rtsp://{}:{}@{}:{}/{}",
            username, password, address, port, stream_path
        )
    }
}

fn generate_klv_packet() -> Vec<u8> {
    fn encode_u16(value: f64, min: f64, max: f64) -> u16 {
        let scale = (value - min) / (max - min);
        (scale * u16::MAX as f64).round() as u16
    }

    fn encode_i64(value: f64, min: f64, max: f64) -> i64 {
        let scale = (value - min) / (max - min);
        (scale * i64::MAX as f64).round() as i64
    }

    let mut pkt = vec![];

    // Heading (0x02)
    pkt.extend([0x02, 0x02]);
    pkt.extend(&encode_u16(182.0, 0.0, 360.0).to_be_bytes());

    // Latitude (0x0D)
    pkt.extend([0x0D, 0x08]);
    pkt.extend(&encode_i64(-30.123456, -90.0, 90.0).to_be_bytes());

    // Longitude (0x0E)
    pkt.extend([0x0E, 0x08]);
    pkt.extend(&encode_i64(-51.123456, -180.0, 180.0).to_be_bytes());

    // Altitude (0x0F)
    pkt.extend([0x0F, 0x04]);
    let alt = ((430.0 / 19999.0) * u32::MAX as f64).round() as u32;
    pkt.extend(&alt.to_be_bytes());

    pkt
}

#[cfg(any(target_os = "windows", target_os = "linux"))]
fn spawn_ffmpeg(
    rtsp_url: String,
    stop_rx: Receiver<()>,
    status_tx: Sender<Result<(), String>>,
) {
    thread::spawn(move || {
        let mut cmd = Command::new("ffmpeg");

        cmd.args(&[
            "-f", "gdigrab",
            "-i", "desktop",
            "-f", "klv",
            "-i", "-",
            "-map", "0:v",
            "-map", "1",
            "-c:v", "libx264",
            "-metadata:s:1", "handler_name=klv_data",
            "-f", "rtsp",
            "-rtsp_transport", "tcp",
            &rtsp_url,
        ]);

        #[cfg(target_os = "windows")]
        let child_result = cmd
            .creation_flags(CREATE_NO_WINDOW)
            .stdin(Stdio::piped())
            .spawn();

        #[cfg(target_os = "linux")]
        let child_result = cmd.stdin(Stdio::piped()).spawn();

        match child_result {
            Ok(mut child) => {
                let _ = status_tx.send(Ok(()));

                if let Some(mut stdin) = child.stdin.take() {
                    let klv = generate_klv_packet();
                    loop {
                        if stop_rx.try_recv().is_ok() {
                            break;
                        }
                        if stdin.write_all(&klv).is_err() {
                            break;
                        }
                        if stdin.flush().is_err() {
                            break;
                        }
                        thread::sleep(Duration::from_secs(1));
                    }
                }

                let _ = child.kill();
            }
            Err(e) => {
                let _ = status_tx.send(Err(format!("Failed to start FFmpeg: {}", e)));
            }
        }
    });
}

pub fn start_stream(
    ctx: Context,
    address: String,
    port: String,
    stream_path: String,
    username: String,
    password: String,
) -> &'static str {
        let (stop_tx, stop_rx) = mpsc::channel();
        let (status_tx, status_rx) = mpsc::channel();

        let rtsp_url = build_rtsp_url(&address, &port, &stream_path, &username, &password);

        spawn_ffmpeg(rtsp_url, stop_rx, status_tx);

        match STREAM_CTRL.lock() {
            Ok(mut lock) => *lock = Some(stop_tx),
            Err(e) => {
                let _ = ctx.callback_data(
                    "VIDEO ERROR",
                    "Failed to acquire lock for stream control",
                    e.to_string(),
                );
                return "stream control lock error";
            }
        }

        match status_rx.recv_timeout(Duration::from_secs(2)) {
            Ok(Ok(())) => {
                let _ = ctx.callback_null("VIDEO", "FFmpeg started successfully");
                "starting video stream"
            }
            Ok(Err(e)) => {
                let _ = ctx.callback_data("VIDEO ERROR", "FFmpeg failed to start", e);
                "ffmpeg failed to start"
            }
            Err(_) => {
                let _ = ctx.callback_null("VIDEO ERROR", "FFmpeg did not respond in time");
                "ffmpeg did not respond"
            }
        }
}

pub fn stop_stream(ctx: Context) -> &'static str {
    match STREAM_CTRL.lock() {
        Ok(mut lock) => {
            if let Some(tx) = lock.take() {
                if let Err(e) = tx.send(()) {
                    let _ = ctx.callback_data(
                        "VIDEO ERROR",
                        "Failed to send stop signal",
                        e.to_string(),
                    );
                    "error sending stop"
                } else {
                    let _ = ctx.callback_null("VIDEO", "Sent stop signal to FFmpeg");
                    "stopping video stream"
                }
            } else {
                let _ = ctx.callback_null("VIDEO ERROR", "Tried to stop a nonexistent stream");
                "no stream to stop"
            }
        }
        Err(e) => {
            let _ = ctx.callback_data(
                "VIDEO ERROR",
                "Failed to acquire lock for stop",
                e.to_string(),
            );
            "lock error"
        }
    }
}
