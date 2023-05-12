use clap::Parser;

mod config;
mod wordlist;

use wordlist::*;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: bool,

    #[arg(short, long, default_value = "wordlist.txt")]
    output: String,
}


fn main() {
    let args = Args::parse();
    let interactive: bool = args.input;

    if interactive {
        let output: String = args.output;
        generate_wordlists(output);
    } else {
        println!("Help...");
    }
}

