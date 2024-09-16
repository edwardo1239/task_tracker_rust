


#[derive(Debug)]
pub struct Config {
    pub comando: String,
    pub dato: String,
}

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
