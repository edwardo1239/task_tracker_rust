use crate::utils::file_utils::{open_file, save_file};
use crate::utils::id::create_unique_id;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs::File;
use std::{collections::HashMap, io::BufReader, time::SystemTime};

#[derive(Serialize, Deserialize)]
pub struct Tarea {
    pub id: String,
    pub description: String,
    pub status: String,
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
            status: "todo".to_string(),
            created_at: datetime,
            update_at: datetime,
        })
    }
    pub fn guardar(&self) -> Result<(), String> {
        // Serializar la tarea a un Value
        let file = open_file()?;

        let mut jsdon_data = Self::crear_json(file)?;

        if let Ok(tarea) = serde_json::to_value(self) {
            jsdon_data.insert(self.id.clone(), tarea);
        }

        save_file(jsdon_data)?;
        Ok(())
    }
    pub fn modificar_tarea(id: &str, new_info:&str) -> Result<(), String>{
        let file = open_file()?;

        let mut jsdon_data = Self::crear_json(file)?;

        if let Some(tarea) = jsdon_data.get_mut(id){
            if let Some(tarea_obj) = tarea.as_object_mut() {
                tarea_obj["description"] = json!(new_info);
                tarea_obj["update_at"] = json!(Utc::now().to_rfc3339());
            }
        } else {
            return Err(format!("No se encontro la tarea con el id: {}", id));
        }

        save_file(jsdon_data)?;
        Ok(())
    }
    pub fn eliminar_tarea(id: &str) -> Result<(), String>{
        let file = open_file()?;

        let mut jsdon_data = Self::crear_json(file)?;

        if jsdon_data.remove(id.trim()).is_some() {
            save_file(jsdon_data)?;
            Ok(())
        } else {
            Err(format!("No se encontró la tarea con el id: {}", id))
        }

    }
    pub fn mark_in_progress(id: &str) -> Result<(), String>{
        let file = open_file()?;

        let mut jsdon_data = Self::crear_json(file)?;
        if let Some(tarea) = jsdon_data.get_mut(id){
            if let Some(tarea_obj) = tarea.as_object_mut() {
                tarea_obj["status"] = json!("in-progress".to_string());
            }
        } else {
            return Err(format!("No se encontro la tarea con el id: {}", id));
        }

        save_file(jsdon_data)?;
        Ok(())
    }

    fn crear_json (file:File) -> Result<HashMap<String, Value>, String>{
        let reader = BufReader::new(file);
        let jsdon_data: HashMap<String, Value> =
            serde_json::from_reader(reader).map_err(|err| format!("Error parsing JSON: {err}"))?;

        Ok(jsdon_data)
    }
}
