use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, http::header::ContentType};
use actix_web::dev::Response;
use actix_web::web::{Json, Path};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::{Deserialize, Serialize};
use url::form_urlencoded::parse;
use url::quirks::username;
use crate::HashMapContainer;
use crate::profilemodel::UserDetails;
use crate::profileservice::{create_sesh, scrape_sesh};

#[post("/")]
async fn create_user_session(data: web::Data<HashMapContainer>, user_info: web::Json<UserDetails>) -> impl Responder {


    let profile_info_res =  match create_sesh(&user_info.username, &user_info.password, data).await {
        Ok(res) => res,
        Err(e) => "{ \"message\": \"Can't retrieve profile info!\" }".to_string(),
    };

    HttpResponse::Ok().insert_header(ContentType::json()).body(profile_info_res)
}

#[get("/{id}")]
async fn get_profile_info(auth: BearerAuth, path: Path<String>, data: web::Data<HashMapContainer>,) -> impl Responder {
    println!("{}", path);

    let profile_info_res =  match scrape_sesh(&path, auth, data).await {
        Ok(res) => res,
        Err(e) => e.to_string(),
    };

    HttpResponse::Ok().insert_header(ContentType::json()).body(profile_info_res)
}