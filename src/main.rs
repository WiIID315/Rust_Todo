//rustc main.rs & ./main
use std::io;

const LISTS: &str = "list";
const EXIT: &str = "exit";

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    'inputs: loop {
        buffer.clear();
        println!("Please input command (type \"exit\" to leave):");
        let _ = stdin.read_line(&mut buffer);
        let input = buffer.trim().to_ascii_lowercase();
        if input == EXIT {
            break 'inputs;
        }
        // println!("{}", buffer);
    }
    println!("Ciao!");
}
