use arma_rs::Context;
use lazy_static::lazy_static;
use std::process::Command;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::Mutex;
use std::thread;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

lazy_static! {
    static ref STREAM_CTRL: Mutex<Option<Sender<()>>> = Mutex::new(None);
}

#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

pub fn start_stream(
    ctx: Context,
    address: String,
    port: String,
    stream_path: String,
    username: String,
    password: String,
) -> &'static str {
    #[cfg(target_os = "linux")]
    {
        return ctx.callback_null(
            "armatak_video_error",
            "Screen capture is only supported on Windows",
        );
    }

    let (tx, rx): (Sender<()>, Receiver<()>) = mpsc::channel();
    let rtsp_url = format!(
        "rtsp://{}:{}@{}:{}/{}",
        username, password, address, port, stream_path
    );
    let rtsp_url_clone = rtsp_url.clone();

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
            &rtsp_url_clone,
        ]);

        #[cfg(target_os = "windows")]
        let mut child = match cmd.creation_flags(CREATE_NO_WINDOW).spawn() {
            Ok(child) => child,
            Err(e) => {
                return ctx.callback_data(
                    "armatak_video_error",
                    "Failed to Start FFmpeg",
                    e.to_string(),
                );
            }
        };

        if rx.recv().is_err() {
            return ctx.callback_null("armatak_video_error", "Error receiving stop signal");
        }

        #[cfg(target_os = "windows")]
        Ok(if let Err(e) = child.kill() {
            return ctx.callback_data(
                "armatak_video_error",
                "Failed to Stop FFmpeg",
                e.to_string(),
            );
        })
    });

    match STREAM_CTRL.lock() {
        Ok(mut lock) => *lock = Some(tx),
        Err(e) => {
            eprintln!("Failed to acquire lock: {}", e);
        }
    }

    "starting video stream"
}

pub fn stop_stream(ctx: Context) -> &'static str {
    match STREAM_CTRL.lock() {
        Ok(mut lock) => {
            if let Some(tx) = lock.take() {
                if let Err(e) = tx.send(()) {
                    let _ = ctx.callback_data(
                        "armatak_video_error",
                        "Failed to send stop signal",
                        e.to_string(),
                    );
                }
            } else {
                let _ =
                    ctx.callback_null("armatak_video_error", "Tried to stop a nonexistent stream");
            }
        }
        Err(e) => {
            let _ = ctx.callback_data(
                "armatak_video_error",
                "Failed to acquire lock",
                e.to_string(),
            );
        }
    }

    "stopping video stream"
}
