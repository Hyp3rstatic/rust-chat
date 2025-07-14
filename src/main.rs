use rustchat::{connect, start_server};
use std::net::TcpListener;

fn port_is_available (addr: &str, port: u16) -> bool {
  match TcpListener::bind((addr, port)) {
    //bind succesful, port is available
    Ok(_) => true,
    //bind failed, port is unavailable
    Err(_) => false,
  }
}

fn main() {
  let port = 8080;
  let addr = "127.0.0.1";

  if port_is_available(addr, port) {
    println!("starting tcp server on port {}", port);
    start_server(addr, &port);
  }
  else {
    println!("connecting to tcp server on port {}", port);
    connect(addr, &port)//.unwrap()
  }
}

#[cfg(test)]
mod test {}
