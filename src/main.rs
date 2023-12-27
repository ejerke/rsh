use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::{env, fs};

fn main() {
    loop {
        // Print the start-of-line char.
        print!("¤ ");
        match stdout().flush() {
            Ok(_) => (),
            Err(_) => (),
        }

        // Read user input.
        let mut arg = String::new();
        match stdin().read_line(&mut arg) {
            Ok(_) => (),
            Err(_) => continue,
        }

        // Handle user input.
        let arg_handler: Vec<&str> = arg.split_whitespace().collect();

        if arg_handler.len() == 0 {
            println!("No command provided. See available commands with 'help'.");
            continue;
        }

        match  arg_handler[0] {
            "cd" => {
                if arg_handler.len() == 1 || arg_handler.len() > 2 {
                    println!("Wrong syntax, use: 'cd [argument]'\n A valid argument is an absolute or relative path.")
                }
                else {
                    cd(arg_handler[1]);
                }
            }
            "ls" => {
                if arg_handler.len() == 1 {
                    ls(".");
                }
                else if arg_handler.len() == 2 {
                    ls(arg_handler[1]);
                }
                else {
                    println!("Wrong syntax, use: 'ls', or 'ls [argument]'\n A valid argument is an absolute or relative path.")
                }
            }
            "pwd" => pwd(),
            "cat" => {
                if arg_handler.len() < 2 || arg_handler.len() > 2 {
                    println!("Wrong syntax, use: 'cat [argument]'\n A valid argument is an absolute path to a file.");
                    continue;
                }
                match cat(arg_handler[1]) {
                    Ok(_) => (),
                    Err(_) => println!("Couldn't open file."),
                };
            },
            "exit" => break,
            "help" => println!("Available commands:\n help   see this text.\n exit   exit the shell.\n cat    output the contents of a file.\n pwd    Print working directory.\n ls     List contents of working directory.\n cd     Change working directory."),
            _ => println!("Unrecognized command, see available commands with 'help'.")


            

        };
    }
}

fn cd(dir: &str) {
    let new_dir = Path::new(dir);

    match env::set_current_dir(new_dir) {
        Ok(_) => (),
        Err(_) => println!("No such file or directory: {}", new_dir.display()),
    }
}

fn ls(dir: &str) {
    match fs::read_dir(dir){
        Err(_) => {
            println!("Invalid directory!");
            return;
        },
        Ok(a) => {
            let paths = a;
            for path in paths {
                println!("{}", path.unwrap().path().display())
            }
        }
    };


}

fn pwd() {
    match env::current_dir() {
        Ok(a) => println!("{}", a.display()),
        Err(_) => println!("Something went wrong..."),
    }
}

fn cat(file_path: &str) -> Result<usize, std::io::Error> {
    let contents = fs::read_to_string(file_path);
    match contents {
        Err(e) => return Err(e),
        Ok(a) => {
            println!("{}", a);
            return Ok(a.len());
        }
    };
}
