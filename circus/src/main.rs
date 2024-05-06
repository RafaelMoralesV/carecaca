use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short = 'H', long, default_value = "localhost")]
    host: String,

    #[arg(short = 'P', long, default_value_t = 3001, value_parser = clap::value_parser!(u16).range(1..))]
    port: u16,
}

fn main() {
    let args = Cli::parse();

    println!("Hello from Circus!");
    println!("Args: {args:?}");
}
