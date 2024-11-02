use clap::Parser;

/// Simple program to respond to "ping"
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    command: String,
}

fn main() {
    let args = Args::parse();

    if args.command == "ping" {
        println!("pong");
    } else {
        println!("Unknown command: {}", args.command);
    }
}
