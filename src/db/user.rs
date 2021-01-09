use crate::{
    models::user::{NewUser, User},
};
use actix_web::{web::Data, FromRequest};
use log::error;
use sqlx::postgres::PgQueryAs;
use sqlx::PgPool;
use sqlx::Result;
use std::{ops::Deref, sync::Arc};
use uuid::Uuid;
struct UserIdentifier {
    username:Option<String>,
    id:Option<Uuid>
}
    async fn update_updated_at(pool:&PgPool,user: UserIdentifier){
        if user.id.is_some() {
           match sqlx::query(
            "update users SET updated_at = current_timestamp where id = $1"
            ).bind(user.id.unwrap())
                .execute(pool)
                .await{
                    Ok(_) =>{},
                    _ => {error!("can not update updated_at")}
                };
        }
        else{
            if user.username.is_some() {
            match sqlx::query(
            "update users SET updated_at = current_timestamp where username = $1"
            )
            .bind(user.username.unwrap())
            .execute(pool).await {
                Ok(_)=>{},
                _=>{error!("can not update updated_at")}
            };
            }
        }
    }
    pub async fn create(pool:&PgPool, new_user: NewUser) -> Result<User> {
        let user = sqlx::query_as::<_, User>(
            "insert into users (username) values ($1) returning *",
        )
        .bind(new_user.username)
        .fetch_one(pool)
        .await?;
        Ok(user)
    }
    pub async fn change_active(pool:&PgPool,uuid:Uuid, active:bool) -> Result<User> {
        let user = sqlx::query_as::<_, User>(
            "update users SET active = $2 where id = $1",
        )
        .bind(uuid)
        .bind(active)
        .fetch_one(pool)
        .await?;
        Ok(user)
    }
    pub async fn find_by_username(pool:&PgPool, username: &str) -> Result<Option<User>> {
        update_updated_at(pool, UserIdentifier{username:Some(username.to_string()),id:None}).await;
        let maybe_user = sqlx::query_as::<_, User>("select * from users where username = $1")
            .bind(username)
            .fetch_optional(pool)
            .await?;

        Ok(maybe_user)
    }

    pub async fn find_by_id(pool:&PgPool, id: Uuid) -> Result<Option<User>> {
        update_updated_at(pool, UserIdentifier{username:None,id:Some(id)}).await;
        let maybe_user = sqlx::query_as::<_, User>("select * from users where id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await?;

        Ok(maybe_user)
    }
    pub async fn delete_user(pool:&PgPool) -> Result<Option<User>> {
        let maybe_user = sqlx::query_as::<_, User>("DELETE FROM users where users.id not null")
            .fetch_optional(pool)
            .await?;

        Ok(maybe_user)
    }
 
    pub async fn get_all(pool: &PgPool) -> sqlx::Result<Vec<User>> {
    let recs = sqlx::query_as::<_, User>(
            r#"
    SELECT *
    FROM users
            "#
        )
    .fetch_all(pool)
    .await?;
    Ok(recs)
}

