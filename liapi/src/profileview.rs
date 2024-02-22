use std::collections::HashMap;
use std::sync::Arc;
use actix_web::{get, post, web, App, HttpResponse, Responder, http::header::ContentType};
use actix_web::web::{Json, Path};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use thirtyfour::WebDriver;
use tokio::sync::Mutex;
use crate::profilemodel::UserDetails;
use crate::profileservice::{create_sesh, scrape_sesh};

#[post("/")]
async fn create_user_session(data: web::Data<Arc<Mutex<HashMap<String,WebDriver>>>>, user_info: web::Json<UserDetails>) -> impl Responder {
    match create_sesh(&user_info.username, &user_info.password, data).await {
        Ok(res) => HttpResponse::Ok().insert_header(ContentType::json()).body(res),
        Err(e) => HttpResponse::BadRequest().insert_header(ContentType::json()).body("{ \"message\": \"Can't retrieve profile info!\" }".to_string()),
    }
}

#[get("/{id}")]
async fn get_profile_info(auth: BearerAuth, path: Path<String>, data: web::Data<Arc<Mutex<HashMap<String,WebDriver>>>>,) -> impl Responder {
    match scrape_sesh(&path, auth, data).await {
        Ok(res) => HttpResponse::Ok().insert_header(ContentType::json()).body(res),
        Err(e) => HttpResponse::BadRequest().insert_header(ContentType::json()).body(e.to_string()),
    }
}