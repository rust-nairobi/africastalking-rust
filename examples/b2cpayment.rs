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
     "name": "Joe Sampler",
     "phoneNumber": "+254702006545",
     "currencyCode": "KES",
     "amount": 5000.10,
     "reason": "SalaryPayment",
     "metadata" : {
       "description" : "May Salary",
       "employeeId" : "123"
     }}
    ]);

    println!(
        "{:?}",
        gateway.mobile_payment_b2c_request("My Online Store", recipients)
    );
}
