use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use actix_web::web;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::Utc;
use jsonwebtoken::{Algorithm, encode, EncodingKey, Header};

use thirtyfour::{
    prelude::{ElementWaitable},
    By, DesiredCapabilities, WebDriver, 
};
use thirtyfour::error::WebDriverError::CustomError;
use tokio::sync::Mutex;

use url::Url;
use crate::profilemodel::{Claims, CompanyTab, LinkedinProfile};

const cover_img_backup : &str = "";
const profile_img_backup : &str = "";

pub async fn create_sesh(email: &str, password : &str, data: web::Data<Arc<Mutex<HashMap<String,WebDriver>>>>) -> Result<(String), Box<dyn Error>> {

    let email_str = email.to_string();

    let driver = match log_in(email, password).await{
        Ok(res) => res,
        Err(e) => {return Err(Box::new(CustomError(e.to_string())))}
    };

    //Creating our JWT Token based off the email and password
    let exp = Utc::now()
        .checked_add_signed(chrono::Duration::minutes(60))
        .expect("valid timestamp")
        .timestamp();

    let exp : usize = exp as usize;

    let claims = Claims{ email: email_str, exp};
    let jwt_secret = b"super_secret"; //Move this to be a const
    let header = Header::new(Algorithm::HS256);

    let mut key_json = String::from("{ \"token\": \"");

    let key_value = encode(&header, &claims, &EncodingKey::from_secret(jwt_secret))?;

    key_json.push_str(&key_value);
    key_json.push_str("\"}");

    let mut map = data.lock().await;
    map.insert(key_value, driver);

    Ok(key_json)
}

pub async fn scrape_sesh(username : &str, auth: BearerAuth, data: web::Data<Arc<Mutex<HashMap<String,WebDriver>>>> ) -> Result<(String), Box<dyn Error>>{

    let mut map = data.lock().await;

    if !map.contains_key(auth.token()) {
        return Err(Box::new(CustomError("{ \"error:\" \"Failed Auth\" }".to_string())));
    }

    let web_driver = map.get(auth.token()).unwrap();

    let result = sesh_scrape(&web_driver, username).await?;
    let result_string = serde_json::to_string(&result).unwrap();

    Ok(result_string)
}

pub async fn log_in(email: &str, password : &str) -> Result<WebDriver, Box<dyn Error>>{
    let caps = DesiredCapabilities::chrome();

    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    driver.maximize_window().await?;

    let url = Url::parse("https://www.linkedin.com/login")?;

    driver.goto(url).await?;
    thread::sleep(Duration::from_secs(1));

    let username_input = driver
        .find(By::Id(r"username"))
        .await?;

    username_input.wait_until().clickable().await?;
    username_input.send_keys(email).await?;

    let password_input = driver
        .find(By::Id(r"password"))
        .await?;

    password_input.wait_until().clickable().await?;
    password_input.send_keys(password).await?;

    driver.find(By::Css("#organic-div > form > div.login__form_action_container > button")).await?.click().await?;

    match driver.find(By::Css("#error-for-password")).await {
        Ok(res) => {
            driver.quit().await?;
            return Err(Box::new(CustomError("{ \"error:\" \"Failed Login\" }".to_string())));
        },
        Err(e) => (),
    }

    //Verify if there is a captcha
    thread::sleep(Duration::from_secs(3));
    match driver.find(By::Css("#captcha-internal")).await {
        Ok(_res) => { //captcha detected
            //We can solve this by hand or using AI models
            driver.quit().await?;
            return Err(Box::new(CustomError("{ \"error:\" \"Captcha triggered, failed creating session\" }".to_string())));
        },
        Err(_e) => (),
    }

    Ok(driver)
}

pub async fn sesh_scrape(driver: &WebDriver, user: &str) -> Result<LinkedinProfile, Box<dyn Error>> {

    let mut user_url: String = String::from("https://www.linkedin.com/in/");
    user_url.push_str(user);
    let url = Url::parse(&user_url)?;

    driver.goto(url).await?;
    thread::sleep(Duration::from_secs(2));

    let mut li_profile = LinkedinProfile::new();

    let cover_image_selector = driver.find(By::Css(r"#profile-background-image-target-image")).await
        .unwrap_or(driver.active_element().await.unwrap());

    let cover_img_value = cover_image_selector.attr("src").await?;

    let profile_content_selector = driver.find(By::Css(r"#profile-content")).await?;
    let profile_ph5 = profile_content_selector.find(By::Css(r"div.ph5.pb5")).await?;

    let profile_img_selector = profile_ph5.find(By::Css(r"div.display-flex")).await?;
    let profile_img_selector = profile_img_selector.find(By::Css(r"img")).await?;
    let profile_img_val = profile_img_selector.attr("src").await?;
    let profile_img_name = profile_img_selector.attr("title").await?;


    let profile_about_val = profile_ph5.find(By::Css(r"div.mt2.relative > div:nth-child(1) > div.text-body-medium.break-words")).await?.text().await?;

    let profile_is_influencer = driver.find(By::Css( r"#profile-content > div > div.scaffold-layout.scaffold-layout--breakpoint-none.scaffold-layout--main-aside.scaffold-layout--single-column.scaffold-layout--reflow.pv-profile > div > div > main > section.artdeco-card")).await;

    let influencer_img = match profile_is_influencer {
        Ok(res) => res.attr("src").await?.unwrap(),
        _ => "null".to_string(),
    };

    let companies = profile_ph5.find_all(By::Css(r"div.mt2.relative > ul > li")).await?;

    for company in companies {
        let company_img_sel = company.find(By::Css(r"img")).await?;
        let company_img_val = company_img_sel.attr("src").await?;

        let company_title = company.find(By::Css(r"div")).await?.text().await?;

        println!("Company Img: {}", company_img_val.clone().unwrap());
        println!("Company Name: {}", company_title);

        li_profile.company_img_title.push( CompanyTab{ img_url : company_img_val.unwrap(), name : company_title} );
    }


    //let experiences = driver.find(By::Css("#profile-content > div > div.scaffold-layout.scaffold-layout--breakpoint-xl.scaffold-layout--main-aside.scaffold-layout--reflow.pv-profile > div > div > main")).await?;


    let experiences = profile_content_selector.find(By::Css(r"main")).await?;



    let experiences = experiences.find_all(By::Css("div.pvs-list__outer-container > ul > li")).await?;

    for experience in experiences{
        //let exp_txt = experience.text().await?;
        match experience.find(By::Css(r".visually-hidden")).await {
            Ok(res) => li_profile.experience.push(res.text().await?),
            _ => continue,
        }
    }

    println!("Cover Image: {}", cover_img_value.clone().unwrap_or(cover_img_backup.to_string()));
    println!("Profile Image: {}", profile_img_val.clone().unwrap_or(profile_img_backup.to_string()));
    println!("Profile Name: {}", profile_img_name.clone().unwrap());
    println!("Profile About: {}", profile_about_val);
    println!("Influencer Img: {}", influencer_img);

    li_profile.cover_img = cover_img_value.unwrap_or(cover_img_backup.to_string());
    li_profile.profile_img = profile_img_val.unwrap_or(profile_img_backup.to_string());
    li_profile.profile_img_name = profile_img_name.unwrap_or("John Doe".to_string());
    li_profile.profile_about = profile_about_val;
    li_profile.influencer_img = influencer_img;

    Ok(li_profile)
}
