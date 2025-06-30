use std::io:: {stdin};
use chat::{connect, serve_ftp, connect_ftp, start_server};

#[tokio::main]
async fn main() {
  let mut input = String::new();
  stdin().read_line(&mut input).unwrap();
  if input == "0\n" {
    std::thread::spawn(|| start_server("127.0.0.1", &8080));
    serve_ftp("127.0.0.1:2121".to_string()).await;
  }
  else if input == "1\n" {
    connect("127.0.0.1", &8080).unwrap()
  }
  else if input == "ftp0\n" {
    serve_ftp("127.0.0.1:2121".to_string()).await;
  }
  else if input == "ftp1\n" {
    connect_ftp("127.0.0.1:2121".to_string());
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
