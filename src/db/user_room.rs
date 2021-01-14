use crate::{models::{user_room::UserRoom}};
use sqlx::PgPool;
use sqlx::Result;
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
            select id from room where room.id = $1
        )
        
        and users_id=(
            select id from users where users.id = $2
        ); 
        ",
    )
    .bind(room_id)
    .bind(users_id)
    .fetch_optional(pool)
    .await?;
    Ok(())
}