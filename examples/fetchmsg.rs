extern crate africastalking_gateway;

use std::env;
use africastalking_gateway::AfricasTalkingGateway;


pub fn main() {
    let username = env::var("AFRICAS_TALKING_USERNAME").unwrap();
    let apikey = env::var("AFRICAS_TALKING_APIKEY").unwrap();
    let gway = AfricasTalkingGateway::new(&username, &apikey, "sandbox");

    println!("{}", gway.fetch_messages(0).unwrap());
}
