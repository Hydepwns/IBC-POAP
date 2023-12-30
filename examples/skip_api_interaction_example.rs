use skip_api::{ApiClient, QueryRequest};

fn main() {
    // Setting up the API client
    let client = ApiClient::new("https://api.stargaze.fi");

    // Making a query request to the Stargaze chain
    let request = QueryRequest::new("stargaze_chain_id", "query_payload");
    let res = client.query(request);

    // Handling and parsing the API response
    match res {
        Ok(response) => {
            println!("Query successful: {:?}", response);
        },
        Err(e) => {
            // Error handling for query failures
            println!("Query failed: {:?}", e);
        }
    }
}
