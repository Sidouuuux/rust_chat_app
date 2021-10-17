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

    //liste de tout les clients dans un vecteur
    let mut clients = vec![];
    //initiation du channel en type string
    let (tx, rx) = mpsc::channel::<String>();

    loop{
        //vérifie l'acceptation de la connection
        if let Ok((mut socket, addr)) = listener.accept() {
            println!("Client : {}", addr);

            //clone du transmetteur
            let tx = tx.clone();

            //on clone le socket pour l'ajouter aux clients
            clients.push(socket.try_clone().expect("Failed to clone the client socket"));

            //This example also shows how to use move, in order to give ownership of values to a thread.
            thread::spawn(move || loop {
                //création d'un vecteur qui contiendra le message
                let mut buff = vec![0; MSG_SIZE];

                //le message sera lu dans le buffer
                match socket.read_exact(&mut buff) {
                    //si la lecturre n'a pas d'erreur
                    Ok(_) => {
                        //on divise le buffer avec les espaces pour avoir un slice de string
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        //on transforme ce slice en une seule string
                        let msg = String::from_utf8(msg).expect("Message invalide");

                        //on affiche le message
                        println!("{}: {:?}", addr, msg);
                        //on envoie le message au receveur grace au transmetteur
                        tx.send(msg).expect("Failed to send the message");
                    },
                    //si on a une erreur non bloquante, on continue
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    //si l'erreur envoyé est bloquante, on coupe la connexion
                    Err(_) => {
                        println!("closing connection with: {}", addr);
                        break;
                    }
            }
    }
}
