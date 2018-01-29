extern crate africastalking_gateway;
extern crate serde_json;

use std::env;
use africastalking_gateway::AfricasTalkingGateway;

pub fn main() {
    let username = env::var("AFRICAS_TALKING_USERNAME").unwrap();
    let apikey = env::var("AFRICAS_TALKING_APIKEY").unwrap();
    let gateway = AfricasTalkingGateway::new(&username, &apikey, "sandbox");

    println!("{:?}", gateway.call("+254702006545", "+254702006545"));

    // check  queue status
    println!("{:?}", gateway.get_queued_calls("+254702006545", None));
}
