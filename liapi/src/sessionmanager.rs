use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use jsonwebtoken::{Algorithm, decode, DecodingKey, Validation};
use thirtyfour::WebDriver;
use tokio::sync::Mutex;
use crate::profilemodel::Claims;

pub async fn session_manager(map : Arc<Mutex<HashMap<String,WebDriver>>>){
    let jwt_secret = b"super_secret";

    loop {
        let mut l_map = map.lock().await;

        for elem in l_map.clone().iter(){
            let elem_token = elem.0;

            let decoded = decode::<Claims>(
                elem_token,
                &DecodingKey::from_secret(jwt_secret),
                &Validation::default(),
            );
            match decoded {
                Ok(res) => (),
                Err(err) => {
                    println!("Token is invalid: {:?}, removing session.", err);

                    match elem.1.close_window().await{
                        Ok(res) => println!("Window closed sucessfully"),
                        Err(err) => println!("ERROR: {}", err),
                    }

                    l_map.remove(elem_token);
                }
            };

        }
        drop(l_map);
        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}