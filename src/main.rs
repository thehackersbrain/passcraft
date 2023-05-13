use clap::Parser;
use colored::Colorize;

mod config;
mod wordlist;
mod munge;
mod banner;

use wordlist::*;
use munge::*;
use banner::*;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
struct Args {
    #[arg(short, long)]
    interactive: bool,

    #[arg(short, long)]
    munge: bool,

    #[arg(short, long, default_value = "wordlist.txt")]
    output: String,

    #[arg(short, long, default_value = "false")]
    wordlist: String,
}


fn main() {
    let args = Args::parse();
    let interactive: bool = args.interactive;
    let munge: bool = args.munge;
    let wordlist: String = args.wordlist;

    banner_p();

    if interactive {
            let output: String = args.output;
            generate_wordlists(output);
        } else if munge {
            if wordlist != String::from("false") {
                let wordlists: Vec<String> = read_wordlist(&wordlist).unwrap();
                let final_wordlist: Vec<String> = leet_speak(wordlists.clone());

                match write_to_file(String::from("munged.txt").as_str(), &final_wordlist) {
                    Ok(()) => println!("\n[{}] {}\t({} words)",
                            "+"
                                .bold()
                                .green(),
                            "Wordlist generated successfully..."
                                .green(),
                            &final_wordlist
                                .len()
                                .to_string()
                                .cyan()),
                    Err(_) => println!("Error while generating the wordlist file..."),
                };
            } else {
                println!("[{}] Specify the wordlist file...", "+".bold().red())
            }
    }
}

