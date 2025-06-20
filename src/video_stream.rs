use arma_rs::Context;
use lazy_static::lazy_static;
use std::process::Command;
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

#[cfg(any(target_os = "windows", target_os = "linux"))]
fn spawn_ffmpeg(
    rtsp_url: String,
    stop_rx: Receiver<()>,
    status_tx: Sender<Result<(), String>>,
) {
    thread::spawn(move || {
        let mut cmd = Command::new("ffmpeg");
        cmd.args(&[
            "-f",
            "gdigrab",
            "-i",
            "desktop",
            "-f",
            "rtsp",
            "-rtsp_transport",
            "tcp",
            &rtsp_url,
        ]);

        #[cfg(target_os = "windows")]
        let child_result = cmd.creation_flags(CREATE_NO_WINDOW).spawn();

        #[cfg(target_os = "linux")]
        let child_result = cmd.spawn();

        match child_result {
            Ok(mut child) => {
                let _ = status_tx.send(Ok(()));
                let _ = stop_rx.recv();
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
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    {
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

    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        ctx.callback_null("VIDEO ERROR", "Screen capture is only supported on Windows");
        "unsupported platform"
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
