#[allow(unused_imports)]

use std::io:: {stdin};
use chat::{connect, start_server, port_is_available};

#[tokio::main]
async fn main() {
  //let mut input = String::new();
  //stdin().read_line(&mut input).unwrap();
  
  if port_is_available(8080) {
    println!("Starting Server on Port 8080");
    start_server("127.0.0.1", &8080);
  }
  else {
    println!("Connecting to Server on Port 8080");
    connect("127.0.0.1", &8080).unwrap()
  }
  /*
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
  use chat::connect;
  
//create a server
//connect to server
//on connect recieve "OK"
//disconnect
//close server

  fn testconn () {
    let stream = connect("127.0.0.1", &9000).unwrap();
  }

}
