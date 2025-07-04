#[allow(unused_imports)]

use std::net::{TcpStream, TcpListener};
use std::io:: {Read, Write, stdin, stdout};
use std::thread::{spawn};
use std::thread;

async fn recieve(mut stream: &TcpStream) {
   let mut buffer: [u8; 1024] = [0; 1024];
   stream.read(&mut buffer).expect("failed to read from client");
   let request = String::from_utf8_lossy(&buffer[..]);
   println!("Recieved: {}", request);
   stdout().flush().unwrap();
}

async fn get_input() -> String {
  let mut input = String::new();
  print!(":");
  stdout().flush().unwrap();
  stdin().read_line(&mut input).unwrap();
  input
}

//have recieve and send be two seperate threads, figure out how to do that with borrow rules
//TODO: See the messages of other clients
pub async fn connect(address: &str, port: &u16) -> Result<(), String> {
  let addr_port: String = format!("{}:{}", address, port);
  let mut stream: TcpStream = TcpStream::connect(addr_port.clone()).map_err(|_| format!("connection to host {} failed",addr_port))?;
  let mut input = String::new();
  println!(" --- Joined TCP Server --- ");
  stdout().flush().unwrap();
  stream.write("OK\n".as_bytes()).expect("failed to write input to server");
  loop {

    recieve(&stream).await;

    input = get_input().await;

    stream.write(input.as_bytes()).expect("failed to write input to server");
    if input == "exit\n" {
      println!("disconnecting");
      break;
    }
    else {
      print!("User Sent Message: {}", input);
    }

    stdout().flush().unwrap();
    input = String::new();
    
  }
  Ok(())
}

//TODO: add mutex for serverside display -- later client side as well; Display the ip:port of clients
pub fn handle_connection(mut stream: TcpStream) {
  let response = "welcome client".as_bytes();
  let client_addr = stream.peer_addr().unwrap();
  println!("Client {}", client_addr);

  stream.write(response).expect("failed to write to client");
  loop {
        
        let mut buffer: [u8; 1024] = [0; 1024];
        stream.read(&mut buffer).expect("failed to read from client");
        let mut request = String::from_utf8_lossy(&buffer[..]);
        request = request.to_string().chars()
            .filter(|c| !['\0'].contains(c))
            .collect();
        println!("Connection Sent Message: {}", request.trim_end());
        if request.trim_end() == "exit".to_string(){
          stream.write("Goodbye!".as_bytes()).expect("failed to write to client");
        }
        else if request.trim_end() == "top".to_string(){
          println!("TOP");
          stream.write("hat!".as_bytes()).expect("failed to write to client");
        }
        else {
          stream.write(response).expect("failed to write to client");
        }

  }
}

pub fn start_server(address: &str, port: &u16) {
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

pub fn port_is_available(port: u16) -> bool {
  match TcpListener::bind(("127.0.0.1", port)) {
    Ok(_) => true,
    Err(_) => false,
  }
}
