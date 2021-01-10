use std::string;

use actix_web::{App, HttpRequest, HttpResponse, Responder, Result, delete, get, http::header::Accept, post, put, web};
use actix_web::web::Data;
use config::get_header;
use serde_json::{Value, json};
use serde::{Deserialize, Serialize};
use log::{debug,error};
use actix_session::Session;
use chrono::prelude::*;
use uuid::Uuid;
use std::sync::Arc;
use sqlx::{PgConnection, PgPool, Pool};
use crate::{config, db, db::user::{change_active, create_user, delete_user, find_by_username, get_all}, models::{room::{NewRoom, Room}, user::NewUser}};
#[derive(Serialize, Deserialize)]
struct IUser {
    name:String,
    id:i32,
    status:bool,
    payload:String,
    date:i64
}
#[derive(Serialize, Deserialize)]
struct IRequest {
    token:String,
}
#[put("/api/users/get")]
async fn get_users(req_body:web::Json<IRequest>,session:Session,pool:Data<PgPool>) -> Result<HttpResponse> {
    match config::verify_jwt(req_body.0.token).await {
        Ok(_) => {},
        _ => return Ok(HttpResponse::Unauthorized().await?),
    };
    match get_all(pool.get_ref()).await {
        Ok(val) => {
            return Ok(HttpResponse::Ok().json(
                json!(val)
            ));
        }
        Err(_) => {
            return Ok(HttpResponse::Unauthorized().await?);
        }
    };
    
}
#[put("/api/users/register")]
async fn register_user(req_body:web::Json<NewUser>,session:Session,pool:Data<PgPool>) -> Result<HttpResponse> {
    match create_user(pool.as_ref(),req_body.0).await {
        Ok(val) => {
            let response:Value= json!(
                {
                    "username":val.username,
                    "active":val.active,
                    "token": match config::generate_jwt(val.id).await {
                        Ok(token) => token,
                        _ => "error".to_string()
                    }
                }
            );
            debug!("User insert sucessful");
            return Ok(HttpResponse::Ok().json(
                response
            ));
        },
        
        Err(_) => {
            error!("User insert error");
            return Ok(HttpResponse::Unauthorized().await?);
        }
    };
}
#[put("/api/users/login")]
async fn login_user(req_body:web::Json<NewUser>,session:Session,pool:Data<PgPool>) -> Result<HttpResponse> {
    match find_by_username(pool.as_ref(),&req_body.username).await {
        Ok(val) => {
            match val {
                Some(user) => {
                    let response:Value= json!(
                    {
                        "username":user.username,
                        "active":user.active,
                        "token": match config::generate_jwt(user.id).await {
                            Ok(token) => token,
                            _ => "error".to_string()
                        }
                    });
                    return Ok(HttpResponse::Ok().json(
                        response
                    ));
            }, 
                None => {},
            };
            return Ok(HttpResponse::BadRequest().await?);
        },
        
        Err(_) => {
            error!("User insert error");
            return Ok(HttpResponse::Unauthorized().await?);
        }
    };
}
#[put("/api/users/logout")]
async fn logout_user(req_body:web::Json<IRequest>,session:Session,pool:Data<PgPool>) -> Result<HttpResponse> {
    match config::verify_jwt(req_body.0.token).await {
        Ok(val) => {
            match change_active(pool.as_ref(),val.claims.sub,false).await {
            Ok(val) => {
                return Ok(HttpResponse::Ok().await?);
            },
            _ => {
                return Ok(HttpResponse::BadRequest().await?);
            }
        }
    },      
    Err(_) => {return Ok(HttpResponse::BadRequest().await?);}
    }
}
#[put("/api/users/empty")]
async fn empty_users(session:Session,pool:Data<PgPool>) -> HttpResponse {
    session.clear();
    let _ = delete_user(&pool);
    HttpResponse::Ok().finish()
}
#[derive(Serialize, Deserialize)]
struct NewRoomRequest {
    token:String,
    name:String
}
#[put("/api/room/create")]
async fn create_room(req_body:web::Json<NewRoomRequest>,session:Session,pool:Data<PgPool>) -> Result<HttpResponse> {
    match config::verify_jwt(req_body.0.token).await {
        Ok(val) => {
                match db::room::create_room(&pool, NewRoom{
                name:req_body.0.name,
                user_id:val.claims.sub
            }).await{
                Ok(val) => return Ok(HttpResponse::Ok().json(val).await?),
                _=>{}
            };
        }
       _=>{}
    }
    Ok(HttpResponse::Unauthorized().await?)
}
#[derive(Serialize, Deserialize)]
struct RoomRequest {
    token:String,
    room_id:Uuid
}
#[put("/api/room/get")]
async fn get_room(req_body:web::Json<IRequest>,session:Session,pool:Data<PgPool>) -> Result<HttpResponse> {
    match config::verify_jwt(req_body.0.token).await {
        Ok(val) => {
                match db::room::get_room(&pool,val.claims.sub).await{
                Ok(val) => return Ok(HttpResponse::Ok().json(val).await?),
                _=>{}
            };
        }
       _=>{}
    }
    Ok(HttpResponse::Unauthorized().await?)
}
#[put("/api/room/join")]
async fn join_room(req_body:web::Json<RoomRequest>,session:Session,pool:Data<PgPool>) -> Result<HttpResponse> {
    match config::verify_jwt(req_body.0.token).await {
        Ok(val) => {
                match db::room::join_room(&pool,val.claims.sub,req_body.0.room_id).await{
                Ok(val) => return Ok(HttpResponse::Ok().await?),
                _=>{}
            };
        }
       _=>{}
    }
    Ok(HttpResponse::Unauthorized().await?)
}

#[put("/api/room/leave")]
async fn leave_room(req_body:web::Json<RoomRequest>,session:Session,pool:Data<PgPool>) -> Result<HttpResponse> {
    match config::verify_jwt(req_body.0.token).await {
        Ok(val) => {
                match db::room::leave_room(&pool,val.claims.sub,req_body.0.room_id).await{
                Ok(_) => return Ok(HttpResponse::Ok().await?),
                Err(_)=>{
                    return Ok(HttpResponse::NotModified().await?)
                }
            };
        }
       _=>{}
    }
    Ok(HttpResponse::Unauthorized().await?)
}
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users);
    cfg.service(register_user);
    cfg.service(empty_users);
    cfg.service(login_user);
    cfg.service(create_room);
    cfg.service(get_room);
    cfg.service(join_room);
    cfg.service(leave_room);
}