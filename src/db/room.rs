use crate::{
    models::user::{ User },
    models::room::{NewRoom, Room},
};
use sqlx::PgPool;
use sqlx::Result;
use uuid::Uuid;

use super::user_room::{attach_room, dettach_room};
struct RoomIdentifier {
    name:Option<String>,
    id:Option<Uuid>
}
pub async fn create_room(pool:&PgPool, new_room: NewRoom) -> Result<Room> {
    let room = sqlx::query_as::<_, Room>(
        "insert into room (name) values ($1) returning *",
    )
    .bind(new_room.name)
    .fetch_one(pool)
    .await?;
    match attach_room(pool,room.id,new_room.user_id).await{
        Err(err)=>Err(err),
        Ok(val) => Ok(room)
    }
}

pub async fn get_room(pool:&PgPool, user_id:Uuid) -> Result<Vec<Room>> { 
    let recs = sqlx::query_as::<_, Room>(
            "
    select room.* from user_room 
    join room on room.id = room_id 
    where users_id 
    In (
        select users.id 
        from users where users.id = $1
    ) 
            "
        )
        .bind(user_id)
    .fetch_all(pool)
    .await?;
    Ok(recs)
}
pub async fn get_user_in_room(pool:&PgPool, room_id: Uuid) -> Result<Vec<User>> { 
    let recs = sqlx::query_as::<_, User>(
            "
    select users.* from user_room 
    join users on users.id = users_id 
    where room_id 
    In (
        select room.id 
        from room where room.id = $1
    ) 
            "
        )
    .bind(room_id)
    .fetch_all(pool)
    .await?;
    Ok(recs)
}
pub async fn join_room(pool:&PgPool, user_id:Uuid ,room_id:Uuid) -> Result<()> { 
    match attach_room(pool,room_id,user_id).await{
        Err(err)=>Err(err),
        Ok(val) => Ok(())
    }
}
pub async fn leave_room(pool:&PgPool, user_id:Uuid ,room_id: Uuid) -> Result<()> { 
    match dettach_room(pool,room_id,user_id).await{
        Err(err)=>Err(err),
        Ok(val) => Ok(())
    }
}
