use std::io::{ErrorKind, Read, Write};
//pour la création du serveur
use std::net::TcpListener;
//pour la création des channels
use std::sync::mpsc;
//pour le multithread
use std::thread;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn main() {
    //création d'un TcpListener à 127.0.0.1:6000
    let listener = TcpListener::bind(LOCAL).expect("Failed to create a listener");
    //le nonblocking permet de rentrer dans un mode d'aacceptation non bloquant,
    //ce qui permet de le laisser constament à l'écoute de nouveau message
    listener.set_nonblocking(true).expect("Failed to initialize non-blocking");

}
