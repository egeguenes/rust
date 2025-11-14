use std::{env, path::PathBuf, os::unix::fs::PermissionsExt, process::Command,
    io::{ self, Write}
};

const COMMANDS: [&str; 5] = ["echo", "exit", "type", "pwd", "cd"];

fn main() {
    let mut line = String::new();

    loop {
        print!("$ ");
        io::stdout().flush().expect("Failed to flush");
        line.clear();

        match io::stdin().read_line(&mut line) {
            Ok(0) => {
                println!();
                break;
            }
            Ok(_) => {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                let mut commands = trimmed.split_whitespace();
                let command = commands.next().unwrap();
                match command {
                    "exit" => break,
                    "echo" => {
                        let output: Vec<&str> = commands.collect();
                        println!("{}", output.join(" "));
                    }
                    "type" => {
                        if let Some(type_next) = commands.next() {
                            println!("{}", type_thing(type_next).1);
                        }
                    }
                    "pwd" => {
                        let cwd = env::current_dir().unwrap();
                        println!("{}", cwd.display());
                    }
                    "cd" => {
                        let target = match commands.next() {
                            Some("~") => env::var_os("HOME").map(PathBuf::from).ok_or("cd: HOME not set"),
                            Some(path) => Ok(PathBuf::from(path)),
                            None => env::var_os("HOME").map(PathBuf::from).ok_or("cd: HOME not set"),
                        };
                        match target {
                            Ok(path_buf) => {
                                if let Err(e) = env::set_current_dir(&path_buf) {
                                    eprintln!("cd: {}: No such file or directory", path_buf.display());
                                }
                            }
                            Err(e) => {
                            eprintln!("{}", e);
                        }
                        }
                    }
                    _ => {
                        let (res_bool, res_str) = type_thing(command);
                        if res_bool {
                            let mut comm = Command::new(command);
                            while let Some(arg) = commands.next() {
                                comm.arg(arg);
                            }
                            comm.status().expect("Cannot run!");
                        } else {
                            println!("{}", res_str);
                        }
                    }
                }
            }
            Err(error) => {
                eprintln!("Some error: {}", error);
                break;
            }
        }
    }
}

fn type_thing(command: &str) -> (bool, String) {
    if !command.is_empty() {
        if COMMANDS.contains(&command) {
            return (true, format!("{} is a shell builtin", command).to_string());
        } else {
            if let Some(path_var) = env::var_os("PATH") {
                for dir in env::split_paths(&path_var) {
                    let mut full_path = dir.clone();
                    full_path.push(command);
                    if full_path.is_file() {
                        if let Ok(metadata) = full_path.metadata() {
                            let perms = metadata.permissions();
                            if perms.mode() & 0o111 != 0 {
                                let return_str = format!("{} is {}", command, full_path.display().to_string());
                                return (true, return_str);
                            }
                        }
                    }
                }
            }
            return (false, format!("{}: not found", command).to_string());
        }
    } else {
        return (false, "Invalid args!".to_string());
    }
}

