use std::{
    error::Error,
    io::{self, Write},
};

use task_tracker_rust::{
    models::config::Config,
    persistence::task_storage::{borrar_tarea, crear_tarea, listar_tareas, mark_done, mark_in_progress, modificar_tarea},
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
                    "update" => {
                        match Config::build(&config.dato.as_str()) {
                            Ok(config_update) => {
                                match modificar_tarea(config_update) {
                                    Ok(_) => println!("Tarea guardada con exito"),
                                    Err(err) => eprintln!("Error {err}")
                                }
                            },
                            Err(err) => eprintln!("{err}")
                        }
                    },
                    "delete" => {
                        match borrar_tarea(&config.dato) {
                            Ok(_) => println!("Tarea eliminada con exito"),
                            Err(err) => eprintln!("Error {err}")
                        }
                    },
                    "mark-in-progress" => {
                        match mark_in_progress(&config.dato) {
                            Ok(_) => println!("Tarea marcada en progreso"),
                            Err(err) => eprintln!("Error {err}")
                        }
                    },
                    "mark-done" => {
                        match mark_done(&config.dato) {
                            Ok(_) => println!("Tarea marcada como terminada"),
                            Err(err) => eprintln!("Error {err}")
                        }
                    }
                    "list" => {
                        match listar_tareas(&&config.dato.as_str()) {
                            Ok(_) => {},
                            Err(err) => eprintln!("Error {err}")
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
