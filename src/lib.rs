use std::net::{TcpStream, TcpListener};
use std::io:: {self, Read, Write, stdin, stdout};
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
  let sender_list = Arc::new(Mutex::new(Vec::<Sender<&str>>::new()));
  let receiver_list = Arc::new(Mutex::new(Vec::<Receiver<&str>>::new()));

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
      /*
      let receiver_list_broadcast_guard = receiver_list.lock().unwrap();
      for receiver in &mut *receiver_list_broadcast_guard {
      */
      for receiver in &mut *receiver_list.lock().unwrap() {
        match receiver.try_recv() {
          Ok(msg) => {
            let _sender_list_lock = {
              for sender in &mut *sender_list.lock().unwrap() {
                let _ = sender.send(msg); //handle this error later
              }
            };
          }
          Err(TryRecvError::Empty) => {
            continue;
          }
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
        let (stream_sender, broadcast_receiver) = channel::<&str>();
        let (broadcast_sender, stream_receiver) = channel::<&str>();

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
fn handle_connection(stream: TcpStream, broadcast_sender: Sender<&str>, broadcast_receiver: Receiver<&str>) {

}

/* connect: connects to a tcp server, and manages the resulting stream, as a client */
pub fn connect (addr: &str, port: &u16) {

}

