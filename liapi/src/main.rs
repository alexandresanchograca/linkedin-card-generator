mod profileview;
mod profileservice;
mod profilemodel;

use std::collections::HashMap;
use std::sync::Arc;
use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use thirtyfour::WebDriver;
use tokio::sync::Mutex;
use crate::profileview::{create_user_session, get_profile_info};

#[derive(Clone)]
struct HashMapContainer(pub Arc<Mutex<HashMap<String,WebDriver>>>);

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let shared_hashmap = HashMapContainer(Arc::new(Mutex::new(HashMap::new())));



    HttpServer::new(move|| {

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
        .bind(("127.0.0.1", 8091))?
        .run()
        .await
}
