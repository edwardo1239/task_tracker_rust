use std::{collections::HashMap, io::BufReader};

use serde_json::Value;

use crate::{models::{config::Config, tarea::Tarea}, utils::file_utils::open_file};

pub fn crear_tarea(tarea: &str) -> Result<(), String> {
    let tarea = Tarea::build(&tarea)?;
    tarea.guardar()?;
    Ok(())
}

pub fn modificar_tarea(config: Config) -> Result<(), String>{
    Tarea::modificar_tarea(&config.comando, &config.dato)?;
    Ok(())
}

pub fn borrar_tarea(id: &str) -> Result<(), String>{
    Tarea::eliminar_tarea(&id)?;
    Ok(())
}

pub fn mark_in_progress (id: &str) -> Result<(), String>{
    Tarea::mark_in_progress(&id)?;
    Ok(())
}

pub fn listar_tareas() -> Result<(), String> {
    let file = open_file()?;
    let reader = BufReader::new(file);
    let json_data: HashMap<String, Value> =
        serde_json::from_reader(reader).map_err(|err| format!("Error parsing JSON: {err}"))?;

    // Convertir el HashMap a una cadena JSON formateada
    let pretty_json = serde_json::to_string_pretty(&json_data)
        .map_err(|err| format!("Error serializing JSON: {err}"))?;

    // Imprimir la cadena JSON formateada
    println!("{}", pretty_json);

    Ok(())
}
