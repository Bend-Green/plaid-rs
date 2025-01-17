use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let response = client
        .item_application_list()
        .access_token("your access token")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
