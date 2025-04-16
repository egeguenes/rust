use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: sha1-cracker <wordlist.txt> <sha1 hash>");
    }
}
