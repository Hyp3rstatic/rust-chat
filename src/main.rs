use std::net::{TcpStream, TcpListener};
use std::io:: {Read, Write, stdin, stdout};
use suppaftp::FtpStream;
use unftp_sbe_fs::ServerExt;

async fn serve_ftp(addr_port: String) {
  let ftp_home = std::env::temp_dir();
  let server = libunftp::Server::with_fs(ftp_home)
      .greeting("Welcome to my FTP server")
      .passive_ports(50000..65535)
      .build()
      .unwrap();
  server.listen(addr_port).await.unwrap();
}

fn connect_ftp(addr_port: String) {
  println!("Trying to connect via ftp");
  let mut ftp_stream = FtpStream::connect(addr_port).unwrap();
  let _ = ftp_stream.login("username", "password").unwrap();
  println!("Current directory: {}", ftp_stream.pwd().unwrap());
  loop {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    if input == "exit\n" {
      println!("exiting FTP");
      break;
    }
    stdout().flush().unwrap();
  }
}

fn connect(address: &str, port: &u16) -> Result<(), String> {
  let addr_port: String = format!("{}:{}", address, port);
  let mut stream: TcpStream = TcpStream::connect(addr_port.clone()).map_err(|_| format!("connection to host {} failed",addr_port))?;
  let mut input = String::new();
  loop {
    stdin().read_line(&mut input).unwrap();
    stream.write(input.as_bytes()).expect("failed to write input to server");
    if input == "exit\n" {
      println!("disconnecting");
      break;
    }
    else if input == "ftp\n" {
      //Stream.write("REQUEST FTP SERVER".as_bytes()).expect("failed to write to server");
      connect_ftp("127.0.0.1:2121".to_string()); 
    }
    else {
      println!("User input: {}", input);
    }
      
    stdout().flush().unwrap();
    input = String::new();
  }
  Ok(())
}

fn handle_connection(mut stream: TcpStream) {
  let response = "welcome client".as_bytes();
  let client_addr = stream.peer_addr().unwrap();
  println!("Client {}", client_addr);
  stream.write(response).expect("failed to write to client");
  loop {
        let mut buffer: [u8; 1024] = [0; 1024];
        stream.read(&mut buffer).expect("failed to read from client");
        let request = String::from_utf8_lossy(&buffer[..]);
        println!("{}", request);
        if request == "exit"{
          stream.write("Goodbye!".as_bytes()).expect("failed to write to client");
        }
  }
}

fn start_server(address: &str, port: &u16) {
  let listener: TcpListener = TcpListener::bind(&format!("{}:{}", address, port)).expect("failed to start server");
  println!("server listening on {}:{}", address, port);
  for stream in listener.incoming() {
    match stream {
      Ok(stream) => {
        std::thread::spawn(|| handle_connection(stream));
      }
      Err(e) => {
        eprintln!("failed to accept connection {}", e);
      }
    }
  }
}

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