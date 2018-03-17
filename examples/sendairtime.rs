extern crate africastalking_gateway;
#[macro_use]
extern crate serde_json;

use std::env;
use africastalking_gateway::AfricasTalkingGateway;

pub fn main() {
    let username = env::var("AFRICAS_TALKING_USERNAME").unwrap();
    let apikey = env::var("AFRICAS_TALKING_APIKEY").unwrap();
    let gateway = AfricasTalkingGateway::new(&username, &apikey, "sandbox");

    let recipients = json!([
                           {
     "phoneNumber": "+254702006545",
     "amount": "KES 500"
     }
    ]);

    println!("{:?}", gateway.send_airtime(&recipients));
}
