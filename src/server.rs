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
            let _ = writeln!(stream, "{} se desconectou!", sender);
        }
    }
}

fn handle_client(mut stream: TcpStream, clients: Clients, server_nick: String) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());

    let nickname = loop {
        let mut nickname = String::new();
        if reader.read_line(&mut nickname).is_err() {
            return; // cliente fechou
        }
        let nickname = nickname.trim().to_string();

        if nickname.is_empty()
            || nickname == server_nick
            || clients.lock().unwrap().contains_key(&nickname)
        {
            let _ = writeln!(
                stream,
                "Já existe um cliente com esse nickname, tente outro."
            );
            continue;
        } else {
            let _ = writeln!(stream, "Ok");
            let _ = writeln!(stream, "Bem-vindo, {}!", nickname);
            break nickname;
        }
    };

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

    println!("Digite seu nickname (servidor):");
    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut server_nick = String::new();
    stdin_lock.read_line(&mut server_nick).unwrap();
    let server_nick = server_nick.trim().to_string();

    let clients_clone = Arc::clone(&clients);
    let server_nick_clone = server_nick.clone();

    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let clients_inner = Arc::clone(&clients_clone);
                    let server_nick_inner = server_nick_clone.clone();
                    thread::spawn(move || {
                        handle_client(stream, clients_inner, server_nick_inner);
                    });
                }
                Err(e) => eprintln!("Erro ao aceitar conexão: {}", e),
            }
        }
    });

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
