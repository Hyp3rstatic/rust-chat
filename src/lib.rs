use std::net::{TcpStream, TcpListener};
use std::io:: {self, Read, Write, stdin, stdout};
use std::thread::{spawn};
use std::sync::{Mutex, mpsc, Arc};
use std::net::{Shutdown};

//TODO: Forward Client writes to server to all connected clients

//TODO: Add Mutexes for writes to stdout

// TODO: Don't break when reading nothing from server
fn recieve(mut stream: &TcpStream) {
   let mut buffer: [u8; 1024] = [0; 1024];
   //stream.read(&mut buffer).expect("failed to read");
   match stream.read(&mut buffer) {
    Ok(0) => {
      println!("no dataL {:?}", buffer);
    }
    Ok(n) => {

    }
    Err(error) => {

    }
   }
   let request = String::from_utf8_lossy(&buffer[..]);
   let writelock = {
    let mut outlock = stdout().lock();
    //println!("Recieved: {}", request);
    let _ = writeln!(&mut outlock, "Recieved: {}", request);
    //stdout().flush().unwrap();
    stdout().flush().unwrap();
   };
}

fn get_input() -> String {
  let mut input = String::new();
  //print!(":");
  let lockwrite = {
    let mut outlock = stdout().lock();
    let _ = write!(&mut outlock, ":");
    //outlock.flush().unwrap();
  };
  //stdout().flush().unwrap();
  stdin().read_line(&mut input).unwrap();
  input
}

pub fn connect(address: &str, port: &u16) -> Result<(), String> {
  let addr_port: String = format!("{}:{}", address, port);
  let mut stream: TcpStream = TcpStream::connect(addr_port.clone()).map_err(|_| format!("connection to host {} failed",addr_port))?;
  let (in_chan, out_chan) = mpsc::channel();
  println!(" --- Joined TCP Server --- ");
  //stdout().flush().unwrap();
  stream.write(format!("Hello from {}\n", stream.local_addr().unwrap()).as_bytes()).expect("failed to write input to server");
  recieve(&stream);
  spawn(move || {
    loop {
      let input = get_input();
      in_chan.send(input).unwrap();
    }
  });
  loop {
    recieve(&stream);
    match out_chan.recv() {
      Ok(input) => {
        //let input_recv = out_chan.recv().unwrap();
        stream.write(input.as_bytes()).expect("failed to write input to server");
        if input == "exit\n" {
          recieve(&stream);
          //sleep(Duration::from_secs(1));
          //println!("disconnecting");
          break;
        } else {
          //print!("User Sent Message: {}", input);
        }
      }
      Err(mpsc::RecvError) => {
      }
    }
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
    println!("buffer: {:?}, len: {}", buffer, buffer.len());
    let mut request = String::from_utf8_lossy(&buffer[..]);
    request = request.to_string().chars()
      .filter(|c| !['\0'].contains(c))
      .collect();
    println!("Connection Sent Message: {}", request.trim_end());
    if request.trim_end() == "exit".to_string() || buffer == [0; 1024] {
      println!("closing client connection {}", client_addr);
      stream.write("Goodbye!".as_bytes()).expect("failed to write to client");
      let _ = stream.shutdown(Shutdown::Both);
      break;
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

//create channel 
pub fn start_server(address: &str, port: &u16) {
  let listener: TcpListener = TcpListener::bind(&format!("{}:{}", address, port)).expect("failed to start server");
  //let connections = [];
  println!("Server listening on {}:{}", address, port);
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
