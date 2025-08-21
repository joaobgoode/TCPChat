use std::{
    io::{self, BufRead, BufReader, Read, Write},
    net::TcpStream,
    thread,
};

fn main() {
    let mut stream =
        TcpStream::connect("127.0.0.1:9000").expect("Não foi possível conectar ao servidor");

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
        } else {
            println!("{}", buffer.trim());
            break;
        }
    }

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

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let msg = line.unwrap();
        if msg.trim().is_empty() {
            continue;
        }
        writeln!(stream, "{}", msg).unwrap();
    }
}
