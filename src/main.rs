use std::net::{TcpStream, TcpListener};
use std::io:: {Read, Write, stdin, stdout};
//use std::time::Instant;
use suppaftp::FtpStream;
//use ftp::{SslContext, SslMethod};

//impl user ids that are updated in one handler
//impl keepalive protocol
//impl ftp

/*#[derive(Debug, Clone, Copy)]
struct conn_id {
  id: u32,
  address: String,
  is_connected: bool,
}*/

fn connect(address: &str, port: &u16) -> Result<(), String> {
  let addr_port: String = format!("{}:{}", address, port);
  let mut stream: TcpStream = TcpStream::connect(addr_port.clone()).map_err(|_| format!("connection to host {}:{} failed", address, port))?;
  let mut input = String::new();
  //let mut now = Instant::now();
  loop {
    stdin().read_line(&mut input).unwrap();
    if input == "exit\n" {
      println!("grapes and stuff");
      break;
    }
    else if input == "ftp\n" {
      let mut ftp_stream = FtpStream::connect(addr_port.clone()).map_err(|_| format!("(FTP) failed to connect to host {}", addr_port))?;
      println!("Current directory: {}", ftp_stream.pwd().unwrap());
    }
    else {
      println!("User input: {}", input);
    }
    stream.write(input.as_bytes()).expect("failed to write input to server");
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
    //let now = Instant::now();
      //if(now.as_seconds() == "5") {
        let mut buffer: [u8; 1024] = [0; 1024];
        stream.read(&mut buffer).expect("failed to read from client");
        let request = String::from_utf8_lossy(&buffer[..]);
        println!("{}", request);
        if request == "exit\n"{
          //conn[itsid].address = "";
          //conn[itsid].is_active = false;
        }
        //if request == "" {
          //break;
        //}
      //}
  }
}

fn start_server(address: &str, port: &u16/*, conns: [conn_id] */) {
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

fn main() {
  let mut input = String::new();
  stdin().read_line(&mut input).unwrap();
  if input == "0\n" {
    /*
    let mut conn_ids: [conn_id; 10] = [conn_id{
      id: 0, 
      address: "",
      is_connected: false,
    }; 10];
    let mut i: u32 = 0;
    for mut entry in conn_ids {
      entry.id = i;
      entry.is_connected = false;
      i += 1;
    }
     */
    start_server("127.0.0.1", &8080/*, conn_ids*/);
  }
  else if input == "1\n" {
    connect("127.0.0.1", &8080).unwrap()
  }
  else {
    println!("geek");
  }
}