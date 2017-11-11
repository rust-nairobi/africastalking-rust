// #![deny(missing_docs,
//         missing_debug_implementations, missing_copy_implementations,
//         trivial_casts, trivial_numeric_casts,
//         unsafe_code,
//         unstable_features,
//         unused_import_braces, unused_qualifications)]
// 
#[macro_use] extern crate hyper;
extern crate reqwest;
extern crate serde;
#[macro_use] extern crate serde_json as json;

use std::io::Read;
use std::error::Error;
use std::collections::HashMap;

use serde::ser::Serialize;
use hyper::header::{Headers, Accept};
header! { (Apikey, "apikey") => [String] }

#[allow(unused_variables)]
trait HttpAccessMethods {
    fn send_request(&self, url: &str, data: Option<HashMap<&str, &str>>) {}
}

#[allow(unused_variables)]
trait UserData {
    fn get_user_data(&self) {}
}

#[derive(Debug)]
pub struct AfricasTalkingGateway {
    username: String,
    api_key: String,
    env: String,
}

impl AfricasTalkingGateway {
    pub fn new (username: &str, api_key: &str, env: &str) -> Self {
        Self {
            username: username.into(),
            api_key: api_key.into(),
            env: env.into(),
        }
    }

    fn get_api_host(&self) -> String {
        if self.env == "sandbox" {
            "https://api.sandbox.africastalking.com".to_owned()
        } else {
            "https://api.africastalking.com".to_owned()
        }
    }

    fn get_user_data_url(&self) -> String {
        format!("{}/version1/user", self.get_api_host())
    }

    fn get_sms_url(&self) -> String {
        format!("{}/version1/messaging", self.get_api_host())
    }

    pub fn get_user_data(&self) -> Result<json::Value, Box<::std::error::Error>> {
        let url = format!("{}?username={}", self.get_user_data_url(), self.username);
        let resp = self.send_request(&url, None)?;
        let val: json::Value = json::from_str(&resp)?;

        Ok(val)
        
    }

    #[allow(unused_variables)]
    pub fn send_message(&self, to: &str, message: &str, from: &str, bulk_sms_mode: bool, enqueue: i32, keyword: &str, link_id: &str, retry_duration_in_hours: i32) -> Result<json::Value, Box<Error>> {
        let params = json!({
            "username": self.username,
            "to": to,
            "message": message,
            "bulkSMSMode": bulk_sms_mode as i32
        });

        let url = self.get_sms_url();
        let resp = self.send_form_data(&url, params)?;
        let val: json::Value = json::from_str(&resp)?;

        Ok(val)
    }

    fn send_request(&self, url: &str, data: Option<HashMap<&str, &str>>) -> Result<String, Box<::std::error::Error>>{
        let mut headers =  Headers::new();
        headers.set(Accept::json());
        headers.set(Apikey(self.api_key.clone()));
        let client = reqwest::Client::new();
        let mut resp = match data {
            Some(map) => {
               client.post(url).json(&map).send()?
            },
            None => client.get(url).headers(headers).send()?,
        };
        
        let mut buf = String::new();
        resp.read_to_string(&mut buf)?;

        Ok(buf)
    }


    fn send_form_data<T: Serialize>(&self, url: &str, data: T) -> Result<String, Box<::std::error::Error>>{
        let mut headers =  Headers::new();
        headers.set(Accept::json());
        headers.set(Apikey(self.api_key.clone()));
        let client = reqwest::Client::new();
        let mut resp = client.post(url).form(&data).headers(headers).send()?;
        
        let mut buf = String::new();
        resp.read_to_string(&mut buf)?;

        Ok(buf)
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
