#[allow(unused_imports)]

use std::io:: {stdin};
use chat::{connect, start_server, port_is_available};

#[tokio::main]
async fn main() {

  let port = 8080;

  if port_is_available(port) {
    println!("Starting TCP Server on Port {}", port);
    start_server("127.0.0.1", &port);
  }
  else {
    println!("Connecting to TCP Server on Port {}", port);
    connect("127.0.0.1", &port).await.unwrap()
  }

}

#[cfg(test)]
mod test {
  use chat::{start_server, connect};
  use std::thread::{spawn};
  
//create a server
//connect to server
//on connect recieve "OK"
//disconnect
//close server
//CREATE TEST ONCE PROGRAM IS MORE FUNCTIONAL
  #[test]
  fn testconn () {

    let port = 9090;

  }

}
