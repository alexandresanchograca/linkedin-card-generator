mod profilemodel;
mod profileservice;
mod profileview;
mod sessionmanager;

use crate::profileview::{create_user_session, get_profile_info};
use crate::sessionmanager::session_manager;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use std::collections::HashMap;
use std::sync::Arc;
use thirtyfour::WebDriver;
use tokio::sync::Mutex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let shared_hashmap: Arc<Mutex<HashMap<String, WebDriver>>> =
        Arc::new(Mutex::new(HashMap::new()));

    //Running session manager to close and remove old sessions
    let shared_map_clone = shared_hashmap.clone();
    tokio::spawn(async move {
        session_manager(shared_map_clone).await;
    });

    HttpServer::new(move || {
        let cors = Cors::default() // Constructs a new CORS middleware builder
            .allow_any_origin() // WARNING: This is not recommended for production use!
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(shared_hashmap.clone()))
            .service(get_profile_info)
            .service(create_user_session)
    })
    .bind(("127.0.0.1", 27014))?
    .run()
    .await
}
