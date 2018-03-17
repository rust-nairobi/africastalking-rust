extern crate africastalking_gateway;

use std::env;
use africastalking_gateway::{AfricasTalkingGateway, SMSMessage};

pub fn main() {
    let username = env::var("AFRICAS_TALKING_USERNAME").unwrap();
    let apikey = env::var("AFRICAS_TALKING_APIKEY").unwrap();
    let gway = AfricasTalkingGateway::new(&username, &apikey, "sandbox");
    let msg = SMSMessage {
        username,
        to: "+254702006545".to_string(),
        message: "hello matt".to_string(),
        ..Default::default()
    };

    println!("{}", gway.send_message(msg).unwrap());
}
