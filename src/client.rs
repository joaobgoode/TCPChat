use std::{
    io::{self, BufRead, BufReader, Write},
    net::TcpStream,
    thread,
};

fn main() {
    let mut stream =
        TcpStream::connect("127.0.0.1:9000").expect("Não foi possível conectar ao servidor");

    println!("Digite seu nickname:");
    let mut nickname = String::new();
    io::stdin().read_line(&mut nickname).unwrap();
    let nickname = nickname.trim();

    writeln!(stream, "{}", nickname).unwrap();

    let stream_clone = stream.try_clone().expect("Falha ao clonar o stream");
    thread::spawn(move || {
        let reader = BufReader::new(stream_clone);
        for line in reader.lines() {
            match line {
                Ok(msg) => {
                    if msg.contains("não está disponível") {
                        eprintln!("{}", msg);
                        std::process::exit(1);
                    }
                    println!("{}", msg);
                }
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
