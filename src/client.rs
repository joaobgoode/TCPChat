use std::{
    io::{self, BufRead, BufReader, Write},
    net::TcpStream,
    thread,
};

fn main() {
    println!("Digite o IP e porta do servidor (ex: 192.168.0.10:9000):");
    let mut server_addr = String::new();
    io::stdin().read_line(&mut server_addr).unwrap();
    let server_addr = server_addr.trim();

    let mut stream = TcpStream::connect(server_addr)
        .unwrap_or_else(|_| panic!("Não foi possível conectar ao servidor em {}", server_addr));

    let mut buffer = String::new();

    loop {
        println!("Digite seu nickname:");
        let mut nickname = String::new();
        io::stdin().read_line(&mut nickname).unwrap();
        let nickname = nickname.trim();

        writeln!(stream, "{}", nickname).unwrap();
        stream.flush().unwrap();

        buffer.clear();
        let mut reader = BufReader::new(&stream);
        reader.read_line(&mut buffer).unwrap();

        if buffer.trim() == "Já existe um cliente com esse nickname, tente outro." {
            println!("{}", buffer.trim());
            continue;
        } else if buffer.trim() == "Ok" {
            println!("Bem-vindo {}!", nickname);
            break;
        } else {
            println!("{}", buffer.trim());
            break;
        }
    }

    // Thread para ouvir mensagens do servidor
    let stream_clone = stream.try_clone().expect("Falha ao clonar o stream");
    thread::spawn(move || {
        let reader = BufReader::new(stream_clone);
        for line in reader.lines() {
            match line {
                Ok(msg) => println!("{}", msg),
                Err(_) => break,
            }
        }
    });

    // Loop para enviar mensagens
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let msg = line.unwrap();
        if msg.trim().is_empty() {
            continue;
        }
        writeln!(stream, "{}", msg).unwrap();
    }
}
