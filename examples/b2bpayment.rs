extern crate africastalking_gateway;
#[macro_use()]
extern crate serde_json;

use std::env;
use africastalking_gateway::AfricasTalkingGateway;


fn main() {
    let username = env::var("AFRICAS_TALKING_USERNAME").unwrap();
    let apikey = env::var("AFRICAS_TALKING_APIKEY").unwrap();

    let gateway = AfricasTalkingGateway::new(&username, &apikey, "sandbox");

    let recipient_payload = json!([
        {
            "username":"Matt Gathu",
            "provider":"PaymentProvider",
            "transferType":"BusinessBuyGoods",
            "destinationChannel":"supplierProviderChannel",
            "destinationAccount":"supplierAccount",
        }
    ]);

    let recipient_metadata = json!([
        {
            "shopId" : "1234",
            "itemId" : "abcdef"
        }
    ]);

    println!(
        "{:?}", gateway.mobile_payment_b2b_request("My Online Store", recipient_payload, "KES", "100", recipient_metadata); 
        );

}