use std::io::{stdin, stdout, Write};
use std::fs;

fn main() {
    
    loop {
        let mut arg = String::new();

        // Print the start-of-line char.
        print!("& ");
        match stdout().flush() {
            Ok(_) => (),
            Err(_) => (),
        }

        // Read user input.
        match stdin()
        .read_line(&mut arg) {
            Ok(_) => (),
            Err(_) => continue,
        }

        // Handle user input.
        let arg_handler: Vec<&str> = arg.split_whitespace().collect();

        match  arg_handler[0] {
            "cat" => {
                if arg_handler.len() < 2 || arg_handler.len() > 2 {
                    println!("Wrong syntax, use: cat [argument]\n A valid argument is an absolute path to a file.");
                    continue;
                }
                match cat(arg_handler[1]) {
                    Ok(_) => (),
                    Err(_) => println!("Couldn't open file."),
                };
            },
            "exit" => break,
            "help" => println!("Available commands:\n help   see this text.\n exit   exit the shell.\n cat   output the contents of a stream."),
            _ => println!("Unrecognized command, see available commands with 'help'.")



        };

    }
}

fn cat(file_path: &str) -> Result<usize, std::io::Error> {
    let contents = fs::read_to_string(file_path);
    match contents {
        Err(e) => return Err(e),
        Ok(a) => {
            println!("{}", a);
            return Ok(a.len())
        }
    };
}
