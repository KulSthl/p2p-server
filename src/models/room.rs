use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Room {
    pub id: Uuid,
    pub name: Option<String>,
    #[serde(skip_serializing)]
    pub active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Validate)]
pub struct NewRoom {
    #[validate(length(min = 3))]
    pub name: String,
    pub user_id:Uuid
}
