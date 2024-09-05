use base64::{engine::general_purpose, Engine as _};
use reqwest::blocking::Client;
use reqwest::header;
use serde::Deserialize;
use std::error::Error;
use std::io::Read;

/// User profile structure to deserialize the API response
#[derive(Deserialize)]
pub struct UserProfile {
    pub display_name: String,
    pub email_address: String,
}

/// Get the user profile, determined by the PAT
pub fn get_user_profile(pat: &str) -> Result<UserProfile, Box<dyn Error>> {
    let client = Client::new();
    let url =
        "https://app.vssps.visualstudio.com/_apis/profile/profiles/me?api-version=7.1-preview.3";
    println!("{}", pat);
    let mut response = client
        .get(url)
        .header(
            header::AUTHORIZATION,
            format!(
                "Basic {}",
                general_purpose::STANDARD.encode(format! {":{}:", pat.trim()})
            ),
        )
        .send()?
        .error_for_status()?;

    let mut response_body = String::new();
    response.read_to_string(&mut response_body);

    println!("Response body: {}", response_body);

    let profile = response.json::<UserProfile>()?;
    Ok(profile)
}
