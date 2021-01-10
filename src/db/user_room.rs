use crate::{models::room::{NewRoom, Room}, models::{user::{NewUser, User}, user_room::UserRoom}};
use actix_web::{web::Data, FromRequest};
use log::error;
use sqlx::{postgres::PgQueryAs, sqlx_macros};
use sqlx::PgPool;
use sqlx::Result;
use std::{ops::Deref, sync::Arc};
use uuid::Uuid;
struct RoomIdentifier {
    name:Option<String>,
    id:Option<Uuid>
}
pub async fn attach_room(pool:&PgPool, room_id:Uuid, users_id:Uuid ) -> Result<UserRoom> {
    let room = sqlx::query_as::<_, UserRoom>(
        "insert into user_room (room_id,users_id) values ($1,$2) returning *",
    )
    .bind(room_id)
    .bind(users_id)
    .fetch_one(pool)
    .await?;
    Ok(room)
}
pub async fn dettach_room(pool:&PgPool, room_id:Uuid, users_id:Uuid ) -> Result<()> {
    sqlx::query_as::<_, UserRoom>(
        "
       DELETE FROM user_room where room_id=(
            select id from room where room.name = 'testRaum'
        )
        
        and users_id=(
            select id from users where users.username = 'test'
        ); 
        ",
    )
    .bind(room_id)
    .bind(users_id)
    .fetch_optional(pool)
    .await?;
    Ok(())
}