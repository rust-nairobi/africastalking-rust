extern crate africastalking_gateway; 

use std::env;
use africastalking_gateway::AfricasTalkingGateway;


pub fn main() {
    let username = env::var("AFRICAS_TALKING_USERNAME").unwrap();
    let apikey = env::var("AFRICAS_TALKING_APIKEY").unwrap();
    let gway = AfricasTalkingGateway::new(&username, &apikey, "production");

    println!("{}", gway.send_message("+254702006545", "hello matt", "AFRICASTKNG", true, 1, "key", "id", 0).unwrap());
}
