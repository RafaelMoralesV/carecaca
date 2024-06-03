mod game_state;

use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    process::ExitCode,
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

fn main() -> ExitCode {
    let Cli { host, port } = Cli::parse();

    let addr = format!("{host}:{port}");

    TcpListener::bind(&addr).map_or_else(
        |_| {
            println!("No se pudo bindear el address especificado.");
            ExitCode::FAILURE
        },
        |listener| {
            println!("Listening on: {addr}");

            let _deck = shitface::deck::Deck::new();

            // accept connections and process them serially
            for stream in listener.incoming() {
                if let Ok(stream) = stream {
                    if let Err(e) = handle_stream(stream) {
                        println!("Error en el stream: {e}");
                    }
                } else {
                    println!("Error al aceptar la conexión");
                }
            }

            ExitCode::SUCCESS
        },
    )
}

//Used to handle incoming messages from the clowns (pun intended)
fn handle_stream(mut stream: TcpStream) -> std::io::Result<()> {
    let peer_addr = stream.peer_addr()?;
    println!("Connected to stream on {peer_addr:?}");

    let mut buffer = String::new();
    let mut reader = BufReader::new(stream.try_clone()?);
    stream.write_all(
        b"Welcome. Please input your name. When you are ready to disconnect, type quit.\n",
    )?;
    stream.flush()?;
    while reader.read_line(&mut buffer)? > 0 {
        if buffer.eq("quit\n") {
            println!("--- Client {peer_addr:?} disconnected ---");
            break;
        }

        println!("{peer_addr:?}: {buffer}");
        buffer.clear();
    }
    Ok(())
}
