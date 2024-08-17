# archaeologist

A Rust wrapper for [Artifacts](https://artifactsmmo.com/), an API-based sandbox MMO.

Generated by [openapi-generator](https://openapi-generator.tech/).

## Example Usage

```rust
use archaeologist::{
    apis::{configuration::Configuration, my_account_api},
    models::GoldBankResponseSchema,
};

#[tokio::main]
async fn main() {
    let mut config = Configuration::new();
    config.bearer_access_token = Some("YOUR_TOKEN".to_owned());

    match my_account_api::get_bank_golds_my_bank_gold_get(&config).await {
        Ok(GoldBankResponseSchema { data }) => println!("{:?}", data),
        Err(err) => panic!("{}", err),
    }
}
```
