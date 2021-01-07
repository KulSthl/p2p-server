use std::string;

use actix_web::{get, post, put, delete, web, HttpResponse,HttpRequest, Responder, Result , App};
use serde_json::json;
use serde::{Deserialize, Serialize};
use log::{debug};
use actix_session::Session;
use chrono::prelude::*;

#[derive(Serialize, Deserialize)]
struct IUser {
    name:String,
    id:i32,
    status:bool,
    payload:String,
    date:i64
}
#[get("/users")]
async fn get_users(session:Session) -> Result<HttpResponse> {
    let users: Option<Vec<IUser>> = session.get("user")?;
    let response;
    if let Some(v) = users {
        response = json!(v);
    } else {
        response = json!([]);
    }
    Ok(HttpResponse::Ok().json(
        &response
    ))
}
#[put("/register")]
async fn register_user(req_body:web::Json<IUser>,session:Session) -> Result<HttpResponse> {
    let counter: i32 = session
        .get::<i32>("counter")
        .unwrap_or(Some(0))
        .map_or(0, |inner| inner);
    session.set("counter", counter+1)?;
    let users: Option<Vec<IUser>> = session.get("user")?;
    let new_user = IUser{
        id:counter,
        name:req_body.name.to_string(),
       status:true,
       payload:req_body.payload.to_string(),
       date:Utc::now().timestamp_millis()
    };
    let response = json!(&new_user);
    if let Some(mut v) = users {
        v.push(new_user);
        session.set("user",v);
    } else {
        session.set("user",vec![new_user]);
    }
    
    // session.set(key, value)
    // session.renew();
    Ok(HttpResponse::Ok().json(
        response
    ))
}
#[put("/empty")]
async fn empty_users(session:Session) -> HttpResponse {
    session.clear();
    HttpResponse::Ok().finish()
}
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users);
    cfg.service(register_user);
    cfg.service(empty_users);
}