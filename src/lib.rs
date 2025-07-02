use std::net::{TcpStream, TcpListener};
use std::io:: {self, Read, Write, stdin, stdout};

//TODO: See the messages of other clients
pub fn connect(address: &str, port: &u16) -> Result<(), String> {
  let addr_port: String = format!("{}:{}", address, port);
  let mut stream: TcpStream = TcpStream::connect(addr_port.clone()).map_err(|_| format!("connection to host {} failed",addr_port))?;
  let mut input = String::new();
  println!(" --- Joined TCP Server --- ");
  loop {
    print!(":");
    io::stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();
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
        let request = String::from_utf8_lossy(&buffer[..]);
        print!("Connection Sent Message: {}", request);
        if request == "exit"{
          stream.write("Goodbye!".as_bytes()).expect("failed to write to client");
        }
  }
}

//TODO: if the server is running, automatically connect, if not, start the server
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
