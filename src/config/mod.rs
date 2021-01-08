use sqlx::postgres::PgPool;
use log::{info};
use actix_web::web::{self, block};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{Utc,Duration};
use std::env;
#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: i64,
    // aud
    // role
    // perms
}
pub async fn db_pool(database_url:String) -> Result<sqlx::Pool<sqlx::PgConnection>, sqlx::Error>  {
    info!("Creating database connection pool.");
    PgPool::builder()
        .connect_timeout(std::time::Duration::from_secs(30))
        .build(&database_url)
        .await
}
pub async fn generate_jwt(user_id: Uuid) -> Result<String, actix_web::error::BlockingError<jsonwebtoken::errors::Error>> {
    let jwt_key = env::var("JWT_SECRET").expect("JWT token missing");
    block(move || {
        let headers = Header::default();
        let encoding_key = EncodingKey::from_secret(jwt_key.as_bytes());
        let now = Utc::now() + Duration::days(100); // Expires in 1 day
        let claims = Claims {
            sub: user_id,
            exp: now.timestamp(),
        };
        encode(&headers, &claims, &encoding_key)
    })
    .await
}

pub async fn verify_jwt(token: String)-> Result<TokenData<Claims>, actix_web::error::BlockingError<jsonwebtoken::errors::Error>> {
    let jwt_key = env::var("JWT_SECRET").expect("JWT token missing");
    block(move || {
        let decoding_key = DecodingKey::from_secret(jwt_key.as_bytes());
        let validation = Validation::default();
        decode::<Claims>(&token, &decoding_key, &validation)
    })
    .await
}
pub fn get_header(header:web::HttpRequest,token:&str) -> Option<String>{
    match header.headers().get("token"){
        Some(val) => match val.to_str(){
            Ok(str) => Some(str.to_string()),
            Err(_) => None
        },
        None => None
    }
}