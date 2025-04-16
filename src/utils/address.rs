use std::net::{IpAddr, UdpSocket};

pub fn get_local_address() -> String {
  fn get_local_ip() -> Result<IpAddr, String> {
      let socket = UdpSocket::bind("0.0.0.0:0").map_err(|e| e.to_string())?;
      socket.connect("8.8.8.8:80").map_err(|e| e.to_string())?;
      socket
          .local_addr()
          .map(|addr| addr.ip())
          .map_err(|e| e.to_string())
  }

  let parsed_data = get_local_ip();

  match parsed_data {
      Ok(ip) => {
          return format!("ws://{}:4152", ip.to_string());
      }
      Err(_) => {
          return "not provided".to_string();
      }
  }
}
