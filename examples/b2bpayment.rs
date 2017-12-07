extern crate africastalking_gateway;
#[macro_use]
extern crate serde_json;

use std::env;
use std::collections::HashMap;
use africastalking_gateway::AfricasTalkingGateway;


fn main() {
    let username = env::var("AFRICAS_TALKING_USERNAME").unwrap();
    let apikey = env::var("AFRICAS_TALKING_APIKEY").unwrap();

    let gateway = AfricasTalkingGateway::new(&username, &apikey, "sandbox");

    let mut recipient_payload: HashMap<&str, &str> =  HashMap::new();

    recipient_payload.insert("username", "Matt Gathu");
    recipient_payload.insert("provider", "PaymentProvider");
    recipient_payload.insert("transferType", "BusinessBuyGoods");
    recipient_payload.insert("destinationChannel", "supplierProviderChannel");
    recipient_payload.insert("destinationAccount", "supplierAccount");

    let mut recipient_metadata: HashMap<&str, &str> =  HashMap::new();

    recipient_metadata.insert("shopId", "1234");
    recipient_metadata.insert("itemId" , "abcdef");

    let amount: f32 = 100;

    println!(
        "{:?}", gateway.mobile_payment_b2b_request("My Online Store", recipient_payload, "KES", amount, recipient_metadata)
        );

}