use plaid::PlaidClient;
use plaid::model::*;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let entity_watchlist_screening_id = "your entity watchlist screening id";
    let response = client
        .watchlist_screening_entity_update(entity_watchlist_screening_id)
        .search_terms(UpdateEntityScreeningRequestSearchTerms {
            country: Some("your country".to_owned()),
            email_address: Some("your email address".to_owned()),
            document_number: Some("your document number".to_owned()),
            legal_name: Some("your legal name".to_owned()),
            entity_watchlist_program_id: "your entity watchlist program id".to_owned(),
            phone_number: Some("your phone number".to_owned()),
            url: Some("your url".to_owned()),
        })
        .assignee("your assignee")
        .status("your status")
        .client_user_id("your client user id")
        .reset_fields(&["your reset fields"])
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
