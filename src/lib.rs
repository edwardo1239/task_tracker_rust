use std::{
    fs::File,
    io::{self, Write},
    path::Path,
    sync::atomic::AtomicUsize,
    time::{SystemTime, UNIX_EPOCH},
};

use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Config {
    pub comando: String,
    pub dato: String,
}

#[derive(Debug)]
pub struct Tarea {
    pub id: String,
    pub description: String,
    pub status: i8,
    pub created_at: DateTime<Utc>,
    pub update_at: DateTime<Utc>,
}

static COUNTER: AtomicUsize = AtomicUsize::new(0);

impl Config {
    pub fn build(input: &str) -> Result<Config, &'static str> {
        let mut parts = input.split_whitespace();

        let comando = parts.next().ok_or("No se proporciono el comando")?;
        let dato = parts.next().ok_or("No se proporciono el dato")?;

        Ok(Config {
            comando: comando.to_string(),
            dato: dato.to_string(),
        })
    }
}

impl Tarea {
    pub fn build(input: &str) -> Result<Tarea, &'static str> {
        let now = SystemTime::now();
        let datetime = now.into();

        let id = create_unique_id();

        Ok(Tarea{
            id:id,
            description:"Nada aun".to_string(),
            status: 1,
            created_at: datetime,
            update_at: datetime,
        })
    }


}

fn create_unique_id() -> String {
    let start = SystemTime::now();
    let since_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let timestamp = since_epoch.as_millis();

    let count = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

    format!("{}-{}", timestamp, count)
}

pub fn leer_data() -> Result<String, &'static str> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(input.trim().to_string()),
        Err(_) => Err("Error obteniendo los datos"),
    }
}

pub fn crear_tarea(tarea: &str) -> Result<(), &'static str> {
    Ok(())
}

pub fn open_file() -> Result<File, String> {
    let path = Path::new("tasks.json");
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => {
            let mut new_file = File::create(&path).map_err(|e| e.to_string())?;
            new_file.write_all(b"{}").map_err(|e| e.to_string())?;
            File::open(&path).map_err(|e| e.to_string())?
        }
    };
    Ok(file)
}
