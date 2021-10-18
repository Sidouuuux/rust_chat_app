use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;
use regex::Regex;

const MSG_SIZE: usize = 32;

fn main() {
    let re = Regex::new(r"^[A-Za-z0-9_ -]*$").unwrap();
    let mut client = TcpStream::connect("127.0.0.1:6000").expect("Failed to create a stream");
    client.set_nonblocking(true).expect("Failed to initialize non-blocking");

    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || loop {
        let mut buff = vec![0; MSG_SIZE];
        match client.read_exact(&mut buff) {
            Ok(_) => {
                let _msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
            },
            Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
            Err(_) => {
                break;
            }
        }

        match rx.try_recv() {
            Ok(msg) => {
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE, 0);
                client.write_all(&buff).expect("writing to socket failed");
            }, 
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break
        }

        thread::sleep(Duration::from_millis(100));
    });

    loop {

        let mut buff = String::new();
        println!("Tapez un message (-q pour quitter) :");
        io::stdin().read_line(&mut buff).expect("Failed to read input");

        let msg = buff.trim().to_string();
        /*#[test]
        fn exploration() {
            //assert!(re.is_match(msg));
        }*/
        println!("{:?}", re.is_match(&msg as &str));
        if !(re.is_match(&msg as &str)){
            println!("Le message n'est pas valide");
        }else if msg == "-q" || tx.send(msg).is_err() {break}
       
    }
    println!("Good bye!");

}