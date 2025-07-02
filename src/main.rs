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
    connect("127.0.0.1", &port).unwrap()
  }
  /*s
  let mut input = String::new();
  stdin().read_line(&mut input).unwrap();
  
  if input == "0\n" {
    start_server("127.0.0.1", &8080);
    //std::thread::spawn(|| start_server("127.0.0.1", &8080));
  }
  else if input == "1\n" {
    connect("127.0.0.1", &8080).unwrap()
  }
  else {
    println!("Invalid Input - Exiting Program");
  }
  */
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
  #[test]
  fn testconn () {

    let port = 9090;

    println!("Starting TCP Server on Port {}", port);
    spawn(|| start_server("127.0.0.1", &port));
    //let stream = connect("127.0.0.1", &port).unwrap();
    //stream.write();
    //connect("127.0.0.1", &port).unwrap();
    
  }

}
