use std::{
    collections::HashMap,
    io::{self, BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

type Clients = Arc<Mutex<HashMap<String, TcpStream>>>;

fn broadcast_message(clients: &Clients, sender: &str, message: &str) {
    let clients = clients.lock().unwrap();
    for (nick, mut stream) in clients.iter() {
        if nick != sender {
            let _ = writeln!(stream, "{}: {}", sender, message);
        }
    }
}

fn broadcast_connection(clients: &Clients, sender: &str) {
    let clients = clients.lock().unwrap();
    for (nick, mut stream) in clients.iter() {
        if nick != sender {
            let _ = writeln!(stream, "{} se conectou!", sender);
        }
    }
}

fn broadcast_disconnection(clients: &Clients, sender: &str) {
    let clients = clients.lock().unwrap();
    for (nick, mut stream) in clients.iter() {
        if nick != sender {
            let _ = writeln!(stream, "{} se disconectou!", sender);
        }
    }
}

fn handle_client(stream: TcpStream, clients: Clients) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());

    let mut nickname = String::new();
    if reader.read_line(&mut nickname).is_err() {
        return;
    }
    let nickname = nickname.trim().to_string();

    println!("{} conectou!", nickname);
    broadcast_connection(&clients, &nickname);
    clients
        .lock()
        .unwrap()
        .insert(nickname.clone(), stream.try_clone().unwrap());

    for line in reader.lines() {
        match line {
            Ok(msg) => {
                println!("{}: {}", nickname, msg);
                broadcast_message(&clients, &nickname, &msg);
            }
            Err(_) => break,
        }
    }

    println!("{} desconectou.", nickname);
    broadcast_disconnection(&clients, &nickname);
    clients.lock().unwrap().remove(&nickname);
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:9000").expect("Não foi possível iniciar o servidor");
    println!("Servidor rodando na porta 9000");

    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    let clients_clone = Arc::clone(&clients);
    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let clients_inner = Arc::clone(&clients_clone);
                    thread::spawn(move || {
                        handle_client(stream, clients_inner);
                    });
                }
                Err(e) => eprintln!("Erro ao aceitar conexão: {}", e),
            }
        }
    });

    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();

    println!("Digite seu nickname (servidor):");
    let mut server_nick = String::new();
    stdin_lock.read_line(&mut server_nick).unwrap();
    let server_nick = server_nick.trim().to_string();

    loop {
        let mut input = String::new();
        if stdin_lock.read_line(&mut input).is_err() {
            break;
        }
        let input = input.trim();
        if input.is_empty() {
            continue;
        }
        broadcast_message(&clients, &server_nick, &input);
    }
}
