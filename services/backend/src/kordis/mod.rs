use base64::{engine::general_purpose, Engine as _};
use regex::Regex;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// use lazy_static::lazy_static;

// This is the only way I have found to create a static vector of strings
// I am looking for a way to store the endpoints in the binary file of the application :/
// TODO: come back to this later once a solution has been found
// lazy_static! {
//     pub static ref API_ENDPOINTS: Vec<&'static str> = vec![
//         "/profile",
//         "/agenda",
//         "/news",
//         "/news/banners",
//         "/:year/grades",
//         "/:year/absences",
//         "/:year/classes",
//         "/classes/:classId/students",
//         "/students/:studentId"
//     ];
// }

// I've set the other fields as optional, I may use the expiration later but for now I don't have
// any use case for it
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KordisToken {
    token: String,
    kind: Option<String>,
    expiration: Option<i64>,
    scope: Option<String>,
}

impl KordisToken {
    pub fn new(
        token: &str,
        kind: Option<String>,
        expiration: Option<i64>,
        scope: Option<String>,
    ) -> KordisToken {
        KordisToken {
            token: token.to_owned(),
            kind,
            expiration,
            scope,
        }
    }

    // TODO: extremely unsafe, take care of all of the unwraps
    pub async fn get_agenda(self: &Self) -> Result<serde_json::Value, &'static str> {
        match get_kordis_api_url("agenda", Some(true)) {
            Some(url) => {
                let client: Client = reqwest::ClientBuilder::new().build().unwrap();
                let authorization_header: String = format!("bearer {}", self.token);
                
                let mut request_headers: header::HeaderMap = header::HeaderMap::new();
                request_headers.insert(
                    reqwest::header::AUTHORIZATION,
                    authorization_header.parse().unwrap(),
                );

                println!("{:#?}", request_headers);
                // TODO: take a week number, convert it to EPOCH time
                let params = [
                    ("start", "1678056275462"),
                    ("end", "1679265052467")
                ];

                let parsed_url = reqwest::Url::parse_with_params(&url, params).unwrap();

                // FIXME: handle this response better, especially the unwrap
                let response = client
                    .get(parsed_url)
                    .headers(request_headers)
                    .send()
                    .await
                    .unwrap()
                    .json::<serde_json::Value>()
                    .await
                    .unwrap();

                Ok(response)
            }, 
                _ => Err("Could not get the agenda endpoint")
        }
    }

    // The way kordis authenticates its users is through the location header, it redirects the user
    // to a new path, inside of the value of the header we find all the data necessary to
    pub fn from_location_header(location: &str) -> Result<KordisToken, &'static str> {
        // This regex matches this type of location header value:
        // access_token={imagine_that_there_is_a_real_token_here}&token_type=bearer&expires_in=604704&scope=account
        let location_re: Regex = Regex::new(r"(?:.*)#(([^=]*)=(.[^&]*)&?)*").unwrap();

        if location_re.is_match(location) {
            // regex to extract the key value pairs presents in the location header
            // FIXME: maybe there is a way of doing it in a single regex expression
            let kv_re: Regex = Regex::new(r"(?:(?:.*)#)?(([^=]*)=(.[^&]*)&?)").unwrap();

            // Using the HashMap was not really necessary I just wanted to try it, still learning
            // rust :)
            let mut map: HashMap<&str, &str> = HashMap::new();

            for group in kv_re.captures_iter(location) {
                let key = group.get(2).unwrap().as_str();
                let value = group.get(3).unwrap().as_str();
                map.insert(key, value);
            }

            KordisToken::from_map(map)
        } else {
            Err("The location header came in with an invalid format.")
        }
    }

    fn from_map(map: HashMap<&str, &str>) -> Result<KordisToken, &'static str> {
        // TODO: only the access_token is considered as required right now, might change this later
        // When I have to play around with JWT tokens eventually
        let required_keys = ["access_token"];

        for key in required_keys {
            if !map.contains_key(key) {
                return Err("The map doesn't contain the required keys");
            }
        }

        // FIXME: meh, there must be a wayyy better way of initializing this
        let mut token: String = "init_token".to_string();

        let mut kind: Option<String> = None;
        let mut scope: Option<String> = None;
        let mut expires_in: Option<i64> = None;

        for (key, value) in map {
            match key {
                "access_token" => token = value.to_string(),
                "kind" => kind = Some(value.to_string()),
                "scope" => scope = Some(value.to_string()),
                "expires_in" => {
                    match value.parse::<i64>() {
                        Ok(exp) => expires_in = Some(exp),
                        Err(_) => (),
                    };
                }
                _ => (),
            };
        }

        Ok(KordisToken::new(&token, kind, expires_in, scope))
    }
}

pub async fn authenticate(username: &str, password: &str) -> Result<KordisToken, &'static str> {
    let kordis_auth_url: String =
        std::env::var("KORDIS_AUTH_URL").expect("The KORDIS_AUTH_URL must be set.");
    let authorization_header = format!("Basic {}", encoded_credentials(username, password));

    // I could have used the reqwest::get() method directly but I may have to customize some
    // headers later on :/
    let client: Client = reqwest::Client::builder().build().unwrap();

    let mut request_headers: header::HeaderMap = header::HeaderMap::new();
    request_headers.insert(
        reqwest::header::AUTHORIZATION,
        authorization_header.parse().unwrap(),
    );

    // FIXME: handle this response better, especially the unwrap
    let response_headers = client
        .get(kordis_auth_url)
        .headers(request_headers)
        .send()
        .await
        .unwrap()
        .headers()
        .clone();

    match response_headers.get("location".to_string()) {
        Some(location) => {
            let location_header: String = location.to_str().unwrap().to_string();
            KordisToken::from_location_header(&location_header)
        }
        None => Err("Could not authenticate the user, please check your credentials and try again ! (if the problem persist feel free to create a GitHub Issue :P)"),
    }
}

// In Elixir it would have been so pretty T-T:
// format!("{}:{}", username, password)
// |> general_purpose::STANDARD_NO_PAD.encode()
fn encoded_credentials(username: &str, password: &str) -> String {
    general_purpose::STANDARD_NO_PAD.encode(format!("{}:{}", username, password))
}

fn get_endpoint(name: &str) -> Option<&'static str> {
    match name {
        "profile" => Some("/profile"),
        "agenda" => Some("/agenda"),
        "news" => Some("/news"),
        "banners" => Some("/news/banners"),
        "grades" => Some("/:year/grades"),
        "absences" => Some("/:year/absences"),
        "classes" => Some("/:year/classes"),
        "students" => Some("/classes/:classId/students"),
        "student" => Some("/students/:studentId"),
        _ => None,
    }
}

// the kordis API has some endpoints that have that `/me` prefix to it :p
// example:
// http://thrustworthy-api.com/endpoint -> http://thrustworthy-api.com/me/endpoint
fn get_my_endpoint(name: &str) -> Option<String> {
    match get_endpoint(name) {
        Some(endpoint) => Some(format!("/me{}", endpoint)),
        _ => None,
    }
}

fn get_kordis_base_url() -> String {
    std::env::var("KORDIS_BASE_URL").expect("KORDIS_BASE_URL must be set.")
}

// FIXME: ugly code repetition, I need to find how to make an `optional` without Option
fn get_kordis_api_url(endpoint: &str, me: Option<bool>) -> Option<String> {

    let endpoint: Option<String> = match me {
        Some(true) => get_my_endpoint(endpoint),
        _ => {
            // I had to wrap the get_endpoint method in a match to convert it to a String
            // The previous match had mismatch arms types: String != &'static str
            match get_endpoint(endpoint) {
                Some(s_endpoint) => Some(s_endpoint.to_string()),
                _ => None
            }
        }
    };

   match endpoint {
        Some(endpoint) => Some(format!("{}{}", get_kordis_base_url(), endpoint)),
        _ => None
    } 
}
