use std::{fs::File, io::Write, path::Path};

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
