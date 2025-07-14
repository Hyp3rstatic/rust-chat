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
      //println!("no dataL {:?}", buffer);
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
pub fn handle_connection(mut stream: Arc<Mutex<TcpStream>>, mut sender: mpsc::Sender<String>, mut connections: Arc<Mutex<Vec<Arc<Mutex<TcpStream>>>>>) {
  let mut stream = stream.lock().unwrap();
  let response = "welcome client".as_bytes();
  let client_addr = stream.peer_addr().unwrap();
  println!("Client {}", client_addr);
  //let message: &str = format!("{}", client_addr).as_str();
  sender.send(format!("{}", client_addr)).unwrap();
  /*
  let connections_lock = {
    let mut stream_copy = Arc::new(Mutex::new(stream));
    connections.lock().unwrap().push(stream_copy);
  };
  */
  stream.write(response).expect("failed to write to client");
  loop {    
    let mut buffer: [u8; 1024] = [0; 1024];
    let _ = {
      stream.read(&mut buffer).expect("failed to read from client");
    };
    //let local_addr = stream.local_addr();
    //println!("buffer: {:?}, len: {}", buffer, buffer.len());
    let mut request = String::from_utf8_lossy(&buffer[..]);
    request = request.to_string().chars()
      .filter(|c| !['\0'].contains(c))
      .collect();
    println!("Connection Sent Message: {}", request.trim_end());
    //TODO: remove stream from connections
    if request.trim_end() == "exit".to_string() || buffer == [0; 1024] {
      println!("closing client connection {}", client_addr);
      stream.write("Goodbye!".as_bytes()).expect("failed to write to client");
      let connections_lock = {
        //let mut stream_copy = Arc::new(Mutex::new(stream));
        println!("{:?}", connections);
        connections.lock().unwrap().retain(|VAL| stream.peer_addr().unwrap() != client_addr);
        println!("{:?}", connections);
      };
      let _ = stream.shutdown(Shutdown::Both);
      break;
    }
    else if request.trim_end() == "top".to_string(){
      println!("TOP");
      stream.write("hat!".as_bytes()).expect("failed to write to client");
    }
    else {

      stream.write(response).expect("failed to write to client");
      sender.send(request.to_string());
    }
    
  }
}


//create channel 
pub fn start_server(address: &str, port: &u16) {
  let (connection_sender, connection_reciever) = mpsc::channel::<String>();
  //let _ = connection_sender.send("hello");
  let listener: TcpListener = TcpListener::bind(&format!("{}:{}", address, port)).expect("failed to start server");
  let mut connections:Arc<Mutex<Vec<Arc<Mutex<TcpStream>>>>> = Arc::new(Mutex::new(Vec::new()));
  println!("Server listening on {}:{}", address, port);
  let mut connections_clone = connections.clone();
  spawn(move || {
    
    loop {
      match connection_reciever.recv() {
        Ok(message) => {
          println!("channel recieved: {}", message);
          println!("{:?}", connections_clone);
          println!("{:?}", connections_clone.lock().unwrap()[0]);
          //println!("{:?}", connections_clone.lock().unwrap()[0].lock().unwrap());
          /*let streamlock = {
            connections_clone.lock().unwrap()[0].lock().unwrap().write("WOAHs".as_bytes()).expect("failed to write to client");
          };
          */
          /*
          for stream in connections_clone.lock().unwrap().iter() {
            println!("EEK");
            stream.lock().unwrap().write(message.as_bytes()).expect("failed to write to client");
            println!("OOK");
          }
          */

        }
        Err(mpsc::RecvError) => {
          println!("AWK");
        }
      }
    }
  });

  for stream in listener.incoming() {
    match stream {
      Ok(stream) => {
        let mut stream = Arc::new(Mutex::new(stream));
        let mut connections_copy = connections.clone();
        let sender = connection_sender.clone();
        connections_copy.lock().unwrap().push(stream.clone()); //Have the stream in handle_connection send a message with a stream clone value for the connections vec
        println!("{:?}", connections_copy);
        let stream_clone = stream.clone();
        spawn(|| handle_connection(stream_clone, sender, connections_copy));
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
