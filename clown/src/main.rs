use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

fn main() {
    let addr = "localhost:3001";

    if let Ok(mut stream) = TcpStream::connect(addr) {
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut buf = String::new();

        reader.read_line(&mut buf).unwrap();
        println!("{buf}");
        buf.clear();
        stream.write_all(b"quit\n").unwrap();
        reader.read_line(&mut buf).unwrap();
        println!("{buf}");
        buf.clear();

        stream.shutdown(std::net::Shutdown::Both).unwrap();
    } else {
        println!("Couldn't connect to server...");
    }
}
