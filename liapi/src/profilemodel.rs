use serde::ser::{SerializeSeq, SerializeStruct};
use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub email: String,
    pub exp: usize,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserDetails {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct ProfileError {
    pub message: String,
}

pub struct CompanyTab {
    pub img_url: String,
    pub name: String,
}

pub struct LinkedinProfile {
    pub cover_img : String,
    pub company_img_title: Vec<CompanyTab>,
    pub experience : Vec<String>,
    pub profile_img : String,
    pub profile_img_name : String,
    pub profile_about : String,
    pub influencer_img : String,
}

impl LinkedinProfile {
    pub fn new() -> LinkedinProfile {
        LinkedinProfile {
            cover_img : String::from(""),
            company_img_title : Vec::new(),
            experience : Vec::new(),
            profile_img : String::from(""),
            profile_img_name : String::from(""),
            profile_about : String::from(""),
            influencer_img : String::from("")
        }
    }
}

impl Serialize for LinkedinProfile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut seq = serializer.serialize_struct("Profile", 7)?;
        seq.serialize_field("cover_img", &self.cover_img)?;
        seq.serialize_field("company_img_title",&self.company_img_title)?;
        seq.serialize_field("experience",&self.experience)?;
        seq.serialize_field("profile_img",&self.profile_img)?;
        seq.serialize_field("profile_name",&self.profile_img_name)?;
        seq.serialize_field("profile_about",&self.profile_about)?;
        seq.serialize_field("influencer_img",&self.influencer_img)?;
        seq.end()
    }
}

impl Serialize for CompanyTab {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut seq = serializer.serialize_struct("CompanyTab", 2)?;
        seq.serialize_field("img_url", &self.img_url)?;
        seq.serialize_field("name",&self.name)?;
        seq.end()
    }
}
