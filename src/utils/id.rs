use std::{
    collections::HashMap,
    io::BufReader,
    time::{SystemTime, UNIX_EPOCH},
};

use serde_json::Value;

use crate::utils::file_utils::{open_id, save_file_id};




pub fn create_unique_id() -> Result<String, String> {
    let count = id_counter()?;
    let start = SystemTime::now();
    let since_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let timestamp = since_epoch.as_millis();


    Ok(format!("{}-{}", timestamp, count.to_string()))
}

fn id_counter() -> Result<i64, String> {
    println!("Entro a esta funcion");
    let file = open_id()?;
    let reader = BufReader::new(file);
    let mut jsdon_data: HashMap<String, Value> =
        serde_json::from_reader(reader).map_err(|err| format!("Error parsing JSON: {err}"))?;

    if let Some(id_value) = jsdon_data.get_mut("ID") {

        if let Some(old_id) = id_value.as_i64(){
            let new_id = old_id + 1;
            
            *id_value = Value::from(new_id);

            save_file_id(jsdon_data)?;

            Ok(new_id)
        } else {
            Ok(0)
        }
    } else {
        Err("No se encontro la clave ID".to_string())
    }


}
