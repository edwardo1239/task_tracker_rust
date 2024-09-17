use std::collections::HashMap;
use std::io::Write;
use std::{fs::File, path::Path};

use serde_json::Value;

pub fn open_file() -> Result<File, String> {
    let path = Path::new("tasks.json");
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => {
            // Crear el archivo nuevo y escribir "{}" en él
            let mut new_file = File::create(&path).map_err(|e| e.to_string())?;
            new_file.write_all(b"{}").map_err(|e| e.to_string())?;
            // Volver a abrir el archivo para su uso posterior
            File::open(&path).map_err(|e| e.to_string())?
        }
    };
    Ok(file)
}

pub fn save_file(jsdon_data: HashMap<String, Value>) -> Result<(), String> {
    let path = Path::new("tasks.json");
    let mut file = File::create(&path).map_err(|err| err.to_string())?;
    serde_json::to_writer(&mut file, &jsdon_data).map_err(|err| err.to_string())?;
    Ok(())
}
