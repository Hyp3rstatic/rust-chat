use rustchat::{connect, start_server, port_is_available};

fn main() {
  let port = 8080;
  let addr = "127.0.0.1";

  if port_is_available(port) {
    println!("Starting TCP Server on Port {}", port);
    start_server(addr, &port);
  }
  else {
    println!("Connecting to TCP Server on Port {}", port);
    connect(addr, &port).unwrap()
  }

}

#[cfg(test)]
mod test {}
