use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};

fn main() {
    let addr = "localhost:3001";

    if let Ok(stream) = TcpStream::connect(addr) {
        let reader = BufReader::new(stream.try_clone().unwrap());

        for line in reader.lines() {
            println!("{}", line.unwrap());
            break;
        }

        stream.shutdown(std::net::Shutdown::Both).unwrap();
    } else {
        println!("Couldn't connect to server...");
    }
}
