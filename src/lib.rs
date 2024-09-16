use std::io;

use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Config {
   pub comando: String,
   pub dato: String
}

pub struct Tarea {
   id: i32,
   description: String,
   status: i8,
   created_at: DateTime<Utc>,
   update_at: DateTime<Utc>
}

impl Config {
    pub fn build(input: &str) -> Result<Config, & 'static str>{
      let mut parts = input.split_whitespace();

      let comando = parts.next().ok_or("No se proporciono el comando")?;
      let dato = parts.next().ok_or("No se proporciono el dato")?;

      Ok(Config {
         comando: comando.to_string(),
         dato: dato.to_string(),
      })
    }
}

pub fn leer_data() -> Result<String, &'static str> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(input.trim().to_string()),
        Err(_) => Err("Error obteniendo los datos"),
    }
}

 