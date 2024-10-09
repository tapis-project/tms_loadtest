#![forbid(unsafe_code)]

use goose::prelude::*;
use goose::goose::GooseMethod;
use reqwest::header::HeaderMap;

#[tokio::main]
async fn main() -> Result<(), GooseError> {
    println!("Starting tms_loadtest");

    GooseAttack::initialize()?
        .register_scenario(scenario!("getclient")
            .register_transaction(transaction!(get_tms_client))
        )
        .register_scenario(scenario!("getversion")
            .register_transaction(transaction!(get_tms_version))
        )
        .execute()
        .await?;

    Ok(())
}

// ------------------------------------------------------------------------------
// get_tms_client:
// ------------------------------------------------------------------------------
/// Get the default test client information.
async fn get_tms_client(user: &mut GooseUser) -> TransactionResult {

    let url = &user.base_url;
    println!("********** base_url: {}", url.as_str());

    // Set the headers needed to issue the get_client call.
    let mut headers = HeaderMap::new();
    headers.insert("X-TMS-TENANT", "test".parse().unwrap());
    headers.insert("X-TMS-CLIENT-ID", "testclient1".parse().unwrap());
    headers.insert("X-TMS-CLIENT-SECRET", "secret1".parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());

    // Use the user parameter to generate a reqwest RequestBuilder tailored to the
    // method and targeing our server.
    let reqbuilder = user.get_request_builder(&GooseMethod::Get, 
                                                        "v1/tms/client/testclient1")?;
    
    // Incorporate the lower level reqwest builder into a GooseRequest.
    let goose_request = GooseRequest::builder()
        // Acquire the headers.
        .set_request_builder(reqbuilder.headers(headers))
        // Build the GooseRequest object.
        .build();

    // Use the user parameter to send the GooseRequest and capture response.
    match user.request(goose_request).await?.response {
        Ok(r) => {
            match r.text().await {
                Ok(content) => {
                    println!("{}", content);
                },
                Err(e) => {
                    return TransactionResult::Err(Box::new(TransactionError::Reqwest(e)));
                }
            };
        },
        Err(e) => {
            return TransactionResult::Err(Box::new(TransactionError::Reqwest(e)));
        }
    };
    //println!("{:#?}", goose_resp);

    Ok(())
}

// ------------------------------------------------------------------------------
// get_tms_version:
// ------------------------------------------------------------------------------
/// A very simple transaction that simply retrieves version information.
async fn get_tms_version(user: &mut GooseUser) -> TransactionResult {
    let goose_resp = user.get("/v1/tms/version").await?;
    match goose_resp.response {
        Ok(r) => {
            let content = match r.text().await 
            {
                Ok(content) => content,
                Err(e) => {
                    return TransactionResult::Err(Box::new(TransactionError::Reqwest(e)));
                }
            };

            println!("*** Version: {}", content);
        },
        Err(e) => {
            println!("*** Error: {}", e);
        }
    }

    Ok(())
}


