use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;
use regex::Regex;

struct Message {
    pseudo: String,
    value: String,
} 

fn main() {
    let re = Regex::new(r"^[A-Za-z0-9_ -]*$").unwrap();
    let mut client = TcpStream::connect("127.0.0.1:6000").expect("Failed to create a stream");
    client.set_nonblocking(true).expect("Failed to initialize non-blocking");

    let mut p = String::new();
    println!("Tapez un pseudo :");
    io::stdin().read_line(&mut p).expect("Failed to read input");
    let pseudo2 = p.trim().to_string();
    let mut p = Message { pseudo: pseudo2, value: "".to_string() };

    let (tx, rx) = mpsc::channel::<Message>();

    thread::spawn(move || loop {
        let mut buff = vec![0; 32];
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
            Ok(p) => {
                let mut buff = p.value.clone().into_bytes();
                buff.resize(32, 0);
                client.write_all(&p.value.as_bytes()).expect("writing to socket failed");
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

        p.value = buff.trim().to_string();

        /*#[test]
        fn exploration() {
            assert!(re.is_match(&msg as &str));
        }*/

        //println!("{:?}", re.is_match(&msg as &str));
        if !(re.is_match(&p.value as &str)){
            println!("Le message n'est pas valide");
        }else if p.value == "-q" || tx.send(p).is_err() {break}
       
    }
    println!("Good bye!");
}
