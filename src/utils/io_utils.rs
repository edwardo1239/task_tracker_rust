use std::io;

pub fn leer_data() -> Result<String, &'static str> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(input.trim().to_string()),
        Err(_) => Err("Error obteniendo los datos"),
    }
}
