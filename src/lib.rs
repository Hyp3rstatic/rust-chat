use std::net::{TcpStream, TcpListener};
use std::io:: {Read, Write, stdin, stdout, Result, ErrorKind};
use std::thread::{spawn};
use std::sync::mpsc::{channel, TryRecvError, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::net::{Shutdown};
use std::collections::{HashMap};

/*
  todos:

  login: enter a unique username at server join
  local-network functionality
  add a prompter for the user input
  create test to see how much the server can handle
*/

/*
  channel identification:
  channels are paired with the client's addr and port when put into the lists
  on disconnects the client's addr and port can be used to be remove associated channels from the list
  ensure that on removal the iteration sequence for connected clients is not disturbed
  Pro Tip: HashMap
*/

/* start_server: start and run a tcp chat server */
pub fn start_server(addr: &str, port: &u16) {
  //bind the server listener to addr and port
  let server_listener = TcpListener::bind(&format!("{}:{}", addr, port)).expect("failed to start server");
  println!("server listening on {}:{}", addr, port);

  //create a sender and receiver list that can be modified across different threads
  //key: port:ip, value: receiver/sender
  //specific vars used by broadcaster thread
  let sender_list = Arc::new(Mutex::new(HashMap::< String, Sender<String> > ::new()));
  let receiver_list = Arc::new(Mutex::new(HashMap::< String, Receiver<String> >::new()));

  //create a sender and receiver list for adding new channels on the listener thread
  let receiver_list_addto = receiver_list.clone();
  let sender_list_addto = sender_list.clone();
  
  /*
    Handling multiple users and their messages:
      involved parties: client, server(connected to client stream), broadcaster(on independent server thread)
        *for supporting many clients and servers; only one broadcaster
      msg written from client to server (stream)
      msg written from server to broadcaster (channel1)
      msg written from broadcaster to every server (channel2)
      msg written from server to client (stream)
  */

  //code for broadcaster
  spawn(move || {
    //have a vec for storing the addrs that should be removed at the end of an iteration
    let mut addr_removal_list = Vec::new();

    loop {
      let _receiver_list_lock = {
        for (addr_recv, receiver) in &mut *receiver_list.lock().unwrap() {
          match receiver.try_recv() {
            //on succesful receive
            Ok(msg) => {
              let _sender_list_lock = {
                for (addr_send, sender) in &mut *sender_list.lock().unwrap() {
                  //msg cloned here in send to avoid lifetime & ownership issues
                  let msg = format!("{}: {}", addr_recv, msg);
                  sender.send(msg.clone()).unwrap();
                }
              };
            }
            //on empty receive
            Err(TryRecvError::Empty) => {
              //println!("HERE"); //debug
              continue;
            }
            //on connected channel disconnect
            Err(TryRecvError::Disconnected) => {
              println!("Here"); //debug
              //add disconnected addr to removal list
              addr_removal_list.push(addr_recv.clone());

              //remove disconnected client from sender list immediately
              let _sender_list_lock = {
                sender_list.lock().unwrap().remove(addr_recv);
              };

              println!("client {} disconnected", addr);
              
              continue;
            },
          }
        }
        //remove all clients in removal list from reciever list
        //done after iteration loop to prevent unpredictable behavior
        for addr in addr_removal_list.iter() {
          let _receiver_list_lock = {
            receiver_list.lock().unwrap().remove(addr);
          };
        }
        addr_removal_list.clear();
      };
    }
  });

  //handle the incoming connections to the server
  for stream in server_listener.incoming() {
    //on connection acceptance success
    match stream {
      Ok(stream) => {
        //create channels for stream-broadcast duplex communication
        let (stream_sender, broadcast_receiver) = channel::<String>();
        let (broadcast_sender, stream_receiver) = channel::<String>();

        //create string of client address:port
        let client_addr = format!("{}", stream.peer_addr().unwrap());

        //add new stream_sender to list of senders
        let _sender_list_lock = {
          sender_list_addto.lock().unwrap().insert(client_addr.clone(), stream_sender);
        };

        //add new stream_receiver to list of receivers
        let _receiver_list_lock = {
          receiver_list_addto.lock().unwrap().insert(client_addr.clone(), stream_receiver);
        };

        //spawn a thread to handle the new connection
        spawn(|| handle_connection(stream, broadcast_sender, broadcast_receiver));
      }
      //on connection acceptance failure
      Err(e) => {
        eprintln!("failed to accept connection {}", e);
      }
    }
  }
}

/* handle_connection: manage a TcpStream stream client as the server */
fn handle_connection(mut stream: TcpStream, broadcast_sender: Sender<String>, broadcast_receiver: Receiver<String>) {
  //create a stream handle for the broadcast_thread
  let mut stream_broadcast_handle = stream.try_clone().unwrap();

  //create channels for receiver-reader thread communication
  let (receiver_sender, reader_receiver) = channel::<String>();

  //spawn a thread to check for broadcast_receiver messages to write to the client
  spawn(move || {
    loop {
        //getting messages from reader thread
        match reader_receiver.try_recv() {
        //on succesful receive
        Ok(msg) => {
          if msg == "shutdown" {
            println!("quitting"); //debug
            break;
          }
        }
        //on empty receive
        Err(TryRecvError::Empty) => {
        }
        //on receiver and reader lost communication... abort now
        Err(TryRecvError::Disconnected) => {
          break;
        },
      }
      //getting messages from broadcaster
      match broadcast_receiver.try_recv() {
        //on succesful receive
        Ok(msg) => {
          stream_broadcast_handle.write(msg.as_bytes()).unwrap();
        }
        //on empty receive
        Err(TryRecvError::Empty) => {
          continue;
        }
        //on broadcaster thread disconnect... everything is over!
        Err(TryRecvError::Disconnected) => {
          break;
        },
      }
    }
  });

  loop {
    //set a buffer size for stream reads
    let mut buf: [u8; 1024] = [0; 1024];
    //read any stream data sent by the client into a buffer
    stream.read(&mut buf).expect("failed to read from client");
    //convert the request data into a utf8 string
    let mut request = String::from_utf8_lossy(&buf[..]);
    //remove the empty buffer from the message
    request = request.to_string().chars()
      .filter(|c| !['\0'].contains(c))
      .collect();

    //on client disconnect
    if buf == [0; 1024] {
      receiver_sender.send("shutdown".to_string());
      let _ = stream.shutdown(Shutdown::Both);
      break;
    }
    /* skipping request validation for now */

    //send message to broadcast thread; also trimming trailing blank characters
    //also sends client_addr
    broadcast_sender.send(request.to_string().clone().trim_end().to_string()).unwrap();
  }
}

/* connect: connects to a tcp server, and manages the resulting stream, as a client */
pub fn connect (addr: &str, port: &u16) -> Result<()>{
  //create client stream
  let mut stream = TcpStream::connect(format!("{}:{}", addr, port).clone())?;
  
  //allow stream reads to server to be non-blocking to enable concurrent reading and writing
  stream.set_nonblocking(true).unwrap();

  //create a stream handle for the input thread
  let mut stream_input_handler = stream.try_clone().unwrap();

  //handle user inputs
  spawn (move || {
    loop {
    //create a new input string to read stdin into
    let mut input = String::new();

    //read stdin line into input
    stdin().read_line(&mut input).unwrap();
    
    //write input to server
    stream_input_handler.write(input.as_bytes()).unwrap();
    }
  });

  loop {
    //set a buffer size for stream reads
    let mut buf: [u8; 1024] = [0; 1024];
    //handling nonblocking stream reads
    match stream.read(&mut buf) {
      //on no bytes read
      Ok(0) => {
        println!("server closed");
        break;
      }
      //on bytes read
      Ok(n) => {
        //convert read data into a utf8 string
        let mut msg = String::from_utf8_lossy(&buf[..]);
        println!("{}", msg);
        stdout().flush().unwrap();
        //print read bytes as string
      }
      //on nothing currently to read
      Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
        continue;
      }
      //on stream read error
      Err(e) => {
        break;
      }
    }
  }

  Ok(())
}

