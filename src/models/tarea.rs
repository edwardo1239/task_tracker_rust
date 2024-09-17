use crate::utils::file_utils::{open_file, save_file};
use crate::utils::id::create_unique_id;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, io::BufReader, time::SystemTime};

#[derive(Serialize, Deserialize)]
pub struct Tarea {
    pub id: String,
    pub description: String,
    pub status: i8,
    pub created_at: DateTime<Utc>,
    pub update_at: DateTime<Utc>,
}

impl Tarea {
    pub fn build(input: &str) -> Result<Tarea, &'static str> {
        let now = SystemTime::now();
        let datetime = now.into();

        let id = create_unique_id();

        Ok(Tarea {
            id: id,
            description: input.to_string(),
            status: 1,
            created_at: datetime,
            update_at: datetime,
        })
    }
    pub fn guardar(&self) -> Result<(), String> {
        // Serializar la tarea a un Value
        let file = open_file()?;

        let reader = BufReader::new(file);
        let mut jsdon_data: HashMap<String, Value> =
            serde_json::from_reader(reader).map_err(|err| format!("Error parsing JSON: {err}"))?;

        if let Ok(tarea) = serde_json::to_value(self) {
            jsdon_data.insert(self.id.clone(), tarea);
        }

        save_file(jsdon_data)?;
        Ok(())
    }
}
