use rchat;

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
  use std::io::{BufRead, BufReader};
  use std::net::{TcpStream, TcpListener};
  use std::io:: {Read, Write, stdin, stdout};
  use suppaftp::FtpStream;
  use unftp_sbe_fs::ServerExt;
  
  fn testconn () {
    let stream = connect("127.0.0.1", &9000).unwrap();
    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    while reader.read_line(&mut line)? > 0 {
      println!("Received: {}", line);
      line.clear();
    }

  testconn();

  }
  //std::thread::spawn(|| start_server("127.0.0.1", &8080));
  //std::thread::spawn(|| connect("127.0.0.1", &9000));

}
