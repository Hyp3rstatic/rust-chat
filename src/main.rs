use std::io:: {stdin};
use chat::{connect, start_server};

#[tokio::main]
async fn main() {
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
    println!("geek");
  }
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
