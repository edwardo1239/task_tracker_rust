use std::{
    error::Error,
    io::{self, Write},
};

use task_tracker_rust::{
    models::config::Config,
    persistence::task_storage::{crear_tarea, listar_tareas},
    utils::io_utils::leer_data,
};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    loop {
        print!("task-cli  ");
        io::stdout().flush()?;

        match leer_data() {
            Ok(input) => match Config::build(&input) {
                Ok(config) => match config.comando.as_str() {
                    "add" => {
                        match crear_tarea(config.dato.as_str()) {
                            Ok(_) => println!("Tarea guardada con exito"),
                            Err(err) => println!("Error {err}")
                        }
                    },
                    "list" => {
                        match listar_tareas() {
                            Ok(_) => (),
                            Err(err) => eprintln!("Error: {err}")
                        }
                    },
                    _ => {
                        println!("Comando no reconocido")
                    }
                },
                Err(err) => eprintln!("{err}"),
            },
            Err(err) => eprintln!("{err}"),
        }
    }
}
