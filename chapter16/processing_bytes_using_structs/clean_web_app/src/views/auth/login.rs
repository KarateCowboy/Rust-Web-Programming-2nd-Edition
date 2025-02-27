use crate::diesel;
use diesel::prelude::*;
use actix_web::{web, HttpResponse, http::header::ContentType, };

use crate::database::DB;
use crate::models::user::user::User;
use crate::json_serialization::{login::Login, login_response::LoginResponse};
use crate::schema::users;
use crate::jwt::JwToken;


pub async fn login(credentials: web::Json<Login>, db: DB) -> HttpResponse {
    let username: String = credentials.username.clone();
    let password: String = credentials.password.clone();

    let users = users::table
        .filter(users::columns::username.eq(username.as_str()))
        .load::<User>(&db.connection).unwrap();

    if users.len() == 0 {
        return HttpResponse::NotFound().await.unwrap()
    } else if users.len() > 1 {
        return HttpResponse::Conflict().await.unwrap()
    }

    match users[0].clone().verify(password) {
        true => {
            let user_id = users[0].clone().id;
            let token = JwToken::new(user_id);
            let raw_token = token.encode();
            let response = LoginResponse{token: raw_token.clone()};
            let body = serde_json::to_string(&response).unwrap();
            HttpResponse::Ok().append_header(("token", raw_token))
                              .content_type(ContentType::json())
                              .body(body)
        },
        false => HttpResponse::Unauthorized().await.unwrap()
    }
}

