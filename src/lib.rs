use std::net::{TcpStream, TcpListener};
use std::io:: {Read, Write, stdin, stdout};
use std::thread::{spawn};
use std::sync::mpsc::{channel, TryRecvError, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::net::{Shutdown};

/* start_server: start and run a tcp chat server */
pub fn start_server(addr: &str, port: &u16) {
  //bind the server listener to addr and port
  let server_listener = TcpListener::bind(&format!("{}:{}", addr, port)).expect("failed to start server");
  println!("server listening on {}:{}", addr, port);

  //create a sender and receiver list that can be modified across different threads
  //specific vars used by broadcaster thread
  let sender_list = Arc::new(Mutex::new(Vec::<Sender<String>>::new()));
  let receiver_list = Arc::new(Mutex::new(Vec::<Receiver<String>>::new()));

  //create a sender and reciever list for adding new channels on the listener thread
  let receiver_list_addto = receiver_list.clone();
  let sender_list_addto = sender_list.clone();
  
  /*
    Handling multiple users and their messages:

    names: client, server, broadcaster
    msg written from client to server (stream)
    msg written from server to broadcaster (channel1)
    msg written from broadcaster to every server (channel2)
    msg written from server to client (stream)
  */

  //code for broadcaster
  spawn(move || {
    let _receiver_list_lock = {
      for receiver in &mut *receiver_list.lock().unwrap() {
        match receiver.try_recv() {
          //on succesful receive
          Ok(msg) => {
            let _sender_list_lock = {
              for sender in &mut *sender_list.lock().unwrap() {
                //msg cloned here in send to avoid lifetime & ownership issues
                let _ = sender.send(msg.clone()); //handle this error later
              }
            };
          }
          //on empty receive
          Err(TryRecvError::Empty) => {
            continue;
          }
          //on connected channel disconnect
          Err(TryRecvError::Disconnected) => {
            //remove sender and receiever from lists
          },
        }
      }
    };
  });

  //handle the incoming connections to the server
  for stream in server_listener.incoming() {
    //on connection acceptance success
    match stream {
      Ok(stream) => {
        //create channels for stream-broadcast duplex communication
        let (stream_sender, broadcast_receiver) = channel::<String>();
        let (broadcast_sender, stream_receiver) = channel::<String>();

        //add new stream_sender to list of senders
        let _sender_list_lock = {
          sender_list_addto.lock().unwrap().push(stream_sender);
        };

        //add new stream_receiver to list of receivers
        let _receiver_list_lock = {
          receiver_list_addto.lock().unwrap().push(stream_receiver);
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

  //spawn a thread to check for broadcast_reciever messages to write to the client
  spawn(move || {
    loop {
      match broadcast_receiver.try_recv() {
        //on succesful receive
        Ok(msg) => {
          let _ = stream_broadcast_handle.write(msg.as_bytes()); //handle error later
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
    let mut buffer: [u8; 1024] = [0; 1024];
    //read any stream data sent by the client into a buffer
    stream.read(&mut buffer).expect("failed to read from client");
    //convert the request data into a utf8 string
    let mut request = String::from_utf8_lossy(&buffer[..]);
    //remove the empty buffer from the message
    request = request.to_string().chars()
      .filter(|c| !['\0'].contains(c))
      .collect();
    //remove any trailing blank characters
    //request = request.trim_end();

    /* skipping request validation for now */

    //send message to broadcast thread
    let _ = broadcast_sender.send(request.to_string().clone()); //handle error later
  }
}

/* connect: connects to a tcp server, and manages the resulting stream, as a client */
pub fn connect (addr: &str, port: &u16) {

}

