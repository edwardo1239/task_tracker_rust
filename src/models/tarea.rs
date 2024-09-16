use chrono::{DateTime, Utc};
use std::time::SystemTime;

use crate::utils::id::create_unique_id;

#[derive(Debug)]
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

        Ok(Tarea{
            id:id,
            description:"Nada aun".to_string(),
            status: 1,
            created_at: datetime,
            update_at: datetime,
        })
    }


}