use std::env;
use std::process::exit as exit;
use std::io::{self, Write};
use std::path::Path;
use std::fs::{remove_file, File, OpenOptions};

fn main() {
    let args = match get_args() {
        Ok(valid) => valid,
        Err(e) => {
            eprintln!("{e}");
            exit(1);
        }
    };

    let file_path = Path::new(&args[1]);
    if !file_path.exists() {
        eprintln!("Path {} does not exist!", file_path.display());
        exit(1);
    }

    let mut file = OpenOptions::new()
        .write(true)
        .open(&args[1])
        .expect("File could not be opened!");

    let file_size = file.metadata().expect("Could not get metadata").len();

    match overwite_file(&mut file, file_size) {
        Ok(_) => println!("Successfully overwrite {}. {} Bytes cleared!", file_path.display(), file_size),
        Err(e) => {
            eprintln!("Failed to overwrite: {e}");
            exit(1);
        }
    }
    remove_file(file_path).expect("File could not removed!");
    println!("Successfully removed file {}", file_path.display());
}

fn get_args() -> Result<Vec<String>, &'static str> {
    let mut args = env::args().collect::<Vec<String>>();
    args[0] = Path::new(&args[0])
        .file_name()
        .expect("file name to executable")
        .to_str()
        .expect("exec name should be valud UTF-8")
        .to_owned();

    if args[0] == "rustrm" {
        if args.len() == 2 {
            return Ok(args)
        } else {
            eprintln!("Usage: {} <file_to_remove>", args[0]);
        }
    }
    Err("Invalid Arguments!")
}

fn overwite_file(file: &mut File, file_size: u64) -> io::Result<()> {
    let buffer = vec![0u8; file_size as usize];
    file.write_all(&buffer).expect("Failed to overwrite");
    Ok(())
}
