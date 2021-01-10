use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(sqlx::FromRow, Serialize)]
pub struct UserRoom {
    pub room_id: Uuid,
    pub users_id: Uuid,
}