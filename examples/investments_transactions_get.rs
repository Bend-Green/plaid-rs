use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let access_token = "your access token";
    let start_date = "your start date";
    let end_date = "your end date";
    let response = client
        .investments_transactions_get(access_token, start_date, end_date)
        .options(InvestmentsTransactionsGetRequestOptions {
            account_ids: Some(vec!["your account ids".to_owned()]),
            offset: Some(1),
            count: Some(1),
        })
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
