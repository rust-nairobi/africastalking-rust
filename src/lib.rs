// #![deny(missing_docs,
//         missing_debug_implementations, missing_copy_implementations,
//         trivial_casts, trivial_numeric_casts,
//         unsafe_code,
//         unstable_features,
//         unused_import_braces, unused_qualifications)]
//
#[macro_use]
extern crate hyper;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_json as json;

use std::io::Read;
use std::error::Error;
use std::collections::HashMap;

use serde::ser::Serialize;
use hyper::header::{Accept, Headers};
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
    user_data_url: String,
    sms_url: String,
    voice_url: String,
    sms_subscription_url: String,
    airtime_url: String,
    mobi_payment_checkout_url: String,
    mobi_payment_b2c_url: String,
    mobi_payment_b2b_url: String,
}

impl AfricasTalkingGateway {
    pub fn new(username: &str, api_key: &str, env: &str) -> Self {
        let api_host = if env == "sandbox" {
            "https://api.sandbox.africastalking.com"
        } else {
            "https://api.africastalking.com"
        };
        let voice_host = if env == "sandbox" {
            "https://voice.sandbox.africastalking.com"
        } else {
            "https://voice.africastalking.com"
        };
        let payments_host = if env == "sandbox" {
            "https://payments.sandbox.africastalking.com"
        } else {
            "https://payments.africastalking.com"
        };

        Self {
            username: username.into(),
            api_key: api_key.into(),
            env: env.into(),
            user_data_url: format!("{}/version1/user", api_host),
            sms_url: format!("{}/version1/messaging", api_host),
            voice_url: format!("{}", voice_host),
            sms_subscription_url: format!("{}/version1/subscription", api_host),
            airtime_url: format!("{}/version1/airtime", api_host),
            mobi_payment_checkout_url: format!("{}/mobile/checkout/request", payments_host),
            mobi_payment_b2c_url: format!("{}/mobile/b2c/request", payments_host),
            mobi_payment_b2b_url: format!("{}/mobile/b2b/request", payments_host),
        }
    }

    pub fn get_user_data(&self) -> Result<json::Value, Box<::std::error::Error>> {
        let url = format!("{}?username={}", self.user_data_url, self.username);
        let resp = self.send_request(&url, None)?;
        let val: json::Value = json::from_str(&resp)?;

        Ok(val)

    }

    #[allow(unused_variables)]
    pub fn send_message(&self,
                        to: &str,
                        message: &str,
                        from: &str,
                        bulk_sms_mode: bool,
                        enqueue: i32,
                        keyword: &str,
                        link_id: &str,
                        retry_duration_in_hours: i32)
                        -> Result<json::Value, Box<Error>> {
        let params = json!({
            "username": self.username,
            "to": to,
            "message": message,
            "bulkSMSMode": bulk_sms_mode as i32
        });

        let resp = self.send_form_data(&self.sms_url, params)?;
        let val: json::Value = json::from_str(&resp)?;

        Ok(val)
    }

    fn send_request(&self,
                    url: &str,
                    data: Option<HashMap<&str, &str>>)
                    -> Result<String, Box<::std::error::Error>> {
        let mut headers = Headers::new();
        headers.set(Accept::json());
        headers.set(Apikey(self.api_key.clone()));
        let client = reqwest::Client::new();
        let mut resp = match data {
            Some(map) => {
                client.post(url)
                    .json(&map)
                    .send()?
            }
            None => {
                client.get(url)
                    .headers(headers)
                    .send()?
            }
        };

        let mut buf = String::new();
        resp.read_to_string(&mut buf)?;

        Ok(buf)
    }


    fn send_form_data<T: Serialize>(&self,
                                    url: &str,
                                    data: T)
                                    -> Result<String, Box<::std::error::Error>> {
        let mut headers = Headers::new();
        headers.set(Accept::json());
        headers.set(Apikey(self.api_key.clone()));
        let client = reqwest::Client::new();
        let mut resp = client.post(url)
            .form(&data)
            .headers(headers)
            .send()?;

        let mut buf = String::new();
        resp.read_to_string(&mut buf)?;

        Ok(buf)
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
