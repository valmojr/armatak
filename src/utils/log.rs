use crate::structs::LogPayload;
use log::{error, info, warn};

pub fn log_info(data: LogPayload) -> String {
  match data.status.as_str() {
      "info" => info!("{}", data.message),
      "warn" => warn!("{}", data.message),
      "error" => error!("{}", data.message),
      _ => error!("{}", "Wrong log call"),
  }
  "logged".to_string()
}