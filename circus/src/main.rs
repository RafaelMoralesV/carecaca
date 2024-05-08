use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short = 'H', long, default_value = "localhost")]
    host: String,

    #[arg(short = 'P', long, default_value_t = 3001, value_parser = clap::value_parser!(u16).range(1..))]
    port: u16,
}

fn main() -> std::io::Result<()> {
    let Cli { host, port } = Cli::parse();

    let addr = format!("{host}:{port}");
    let listener = TcpListener::bind(&addr).expect("No se pudo bindear el address especificado.");

    println!("Listening on: {}", addr);

    // accept connections and process them serially
    for stream in listener.incoming() {}

    Ok(())
}

//Used to handle incoming messages from the clowns
fn handle_stream(mut stream: TcpStream) -> std::io::Result<()> {
    let peer_addr = stream.peer_addr()?;
    println!("Connected to stream on {:?}", peer_addr);

    let mut buffer = String::new();
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    stream
        .write(b"Welcome. Please input your name. When you are ready to disconnect, type quit.\n")
        .unwrap();
    stream.flush().unwrap();
    while reader.read_line(&mut buffer).unwrap() > 0 {
        if buffer.eq("quit\n") {
            println!("--- Client {:?} disconnected ---", peer_addr);
            break;
        }

        println!("{:?}: {}", peer_addr, buffer);
        buffer.clear();
    }
    Ok(())
}
