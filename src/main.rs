use std::{error::Error, io::{self, Write}};

use task_tracker_rust::{leer_data, Config};



fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run () -> Result<(), Box<dyn Error>>{
    loop {
        print!("task-cli  ");
        io::stdout().flush()?;

        match leer_data() {
            Ok(input) => {
                match Config::build(&input) {
                    Ok(config) => {
                        match config.comando.as_str() {
                            "add" => {
                                println!("Hola add")
                            },
                            _ => {
                                println!("Comando no reconocido")
                            }
                        }
                    },
                    Err(err) => eprintln!("{err}")
                }
            },
            Err(err) => eprintln!("{err}")
        }
    }
}