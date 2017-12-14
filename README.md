## africastalking-rust

[![Build Status](https://travis-ci.org/rust-nairobi/africastalking-rust.svg?branch=master)](https://travis-ci.org/rust-nairobi/africastalking-rust)

A Rust library for communicating with the Africa's Talking REST API.


### installation

```
cargo install --git https://github.com/rust-nairobi/africastalking-rust
```

### sending a message

```rust
extern crate africastalking_gateway;

use std::env;
use africastalking_gateway::AfricasTalkingGateway;

pub fn main() {
    let username = "your-account-username";
    let apikey = "your-api-key";
    let env = "sandbox";
    let gway = AfricasTalkingGateway::new(&username, &apikey, env);

    println!(
        "{}",
        gway.send_message(
            "+254702xxxxxx",
            "hello Rustacean",
            None, // these are optional parameters
            None,
            None,
            None,
            None,
            None
        ).unwrap()
    );
}
```

### fetching messages

```rust
extern crate africastalking_gateway;

use std::env;
use africastalking_gateway::AfricasTalkingGateway;

pub fn main() {
    let username = "your-account-username";
    let apikey = "your-api-key";
    let env = "sandbox";
    let gway = AfricasTalkingGateway::new(&username, &apikey, &env);

    println!("{}", gway.fetch_messages(0).unwrap());
}
```

### making a voice call

```rust
extern crate africastalking_gateway;
extern crate serde_json;

use std::env;
use africastalking_gateway::AfricasTalkingGateway;

pub fn main() {
    let username = "your-account-username";
    let apikey = "your-api-key";
    let env = "sandbox";
    let gway = AfricasTalkingGateway::new(&username, &apikey, &env);

    println!("{:?}", gateway.call("+254702xxxxxx", "+254702xxxxxx"));

    // check  queue status
    println!("{:?}", gateway.get_queued_calls("+254702xxxxxx", None));
}
```


### sending airtime

```rust
extern crate africastalking_gateway;
#[macro_use]
extern crate serde_json;

use std::env;
use africastalking_gateway::AfricasTalkingGateway;

pub fn main() {
    let username = "your-account-username";
    let apikey = "your-api-key";
    let env = "sandbox";
    let gway = AfricasTalkingGateway::new(&username, &apikey, &env);

    let recipients = json!([
                           {
     "phoneNumber": "+254702xxxxxx",
     "amount": "KES 500"
     }
    ]);

    println!("{:?}", gateway.send_airtime(recipients));
}
```

## license

This project is license used the MIT license. See [LICENSE](LICENSE) for more details.
