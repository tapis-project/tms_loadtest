#![forbid(unsafe_code)]

use std::env;
use std::collections::HashMap;

use lazy_static::lazy_static;

use goose::prelude::*;
use goose::goose::GooseMethod;
use reqwest::header::HeaderMap;

// ***************************************************************************
//                             Static Variables
// ***************************************************************************
// ---------------------------------------------------------------------------
// RuntimeCtx:
// ---------------------------------------------------------------------------
#[derive(Debug)]
#[allow(dead_code)]
pub struct RuntimeCtx {
    pub env_vars: HashMap<&'static str, String>,
}

// Lazily initialize our runtime context with a 'static lifetime.
lazy_static! {
    static ref RUNTIME_CTX: RuntimeCtx = init_runtime_context();
}

// ---------------------------------------------------------------------------
// main:
// ---------------------------------------------------------------------------
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

// ******************************************************************************
//                               Constants
// ******************************************************************************
// Environment variable names.
const X_TMS_TENANT: &str = "X_TMS_TENANT";
const X_TMS_CLIENT_ID: &str = "X_TMS_CLIENT_ID";
const X_TMS_CLIENT_SECRET: &str = "X_TMS_CLIENT_SECRET";
const X_TMS_ADMIN_ID: &str = "X_TMS_ADMIN_ID";
const X_TMS_ADMIN_SECRET: &str = "X_TMS_ADMIN_SECRET";
const TMS_VERBOSE: &str = "TMS_VERBOSE";                  // default is false
const TMS_PARSE_RESPONSE: &str = "TMS_PARSE_RESPONSE";    // default is false 

// ******************************************************************************
//                           Transaction Functions
// ******************************************************************************
// ------------------------------------------------------------------------------
// get_tms_client:
// ------------------------------------------------------------------------------
/// Get the default test client information.
async fn get_tms_client(user: &mut GooseUser) -> TransactionResult {

    // Get custom settings from the environment.
    let env_vars = &RUNTIME_CTX.env_vars;
    let verbose = env_vars.get(TMS_VERBOSE).unwrap();
    let parse_response = env_vars.get(TMS_PARSE_RESPONSE).unwrap();

    // TMS inputs.
    let tenant = env_vars.get(X_TMS_TENANT)
        .unwrap_or_else(|| panic!("* FATAL ERROR: Required environment variable '{}' is not set.", X_TMS_TENANT));
    let client_id = env_vars.get(X_TMS_CLIENT_ID)
        .unwrap_or_else(|| panic!("* FATAL ERROR: Required environment variable '{}' is not set.", X_TMS_CLIENT_ID));
    let client_secret = env_vars.get(X_TMS_CLIENT_SECRET)
        .unwrap_or_else(|| panic!("* FATAL ERROR: Required environment variable '{}' is not set.", X_TMS_CLIENT_SECRET));

    // Set the headers needed to issue the get_client call.
    let mut headers = HeaderMap::new();
    headers.insert("X-TMS-TENANT", tenant.parse().unwrap());
    headers.insert("X-TMS-CLIENT-ID", client_id.parse().unwrap());
    headers.insert("X-TMS-CLIENT-SECRET", client_secret.parse().unwrap());
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
            if parse_response != "false" {
                match r.text().await {
                    Ok(content) => {
                        if verbose != "false" {println!("*** Client: {}", content);}
                    },
                    Err(e) => {
                        return TransactionResult::Err(Box::new(TransactionError::Reqwest(e)));
                    }
                };
            }
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
    // Get custom settings from the environment.
    let env_vars = &RUNTIME_CTX.env_vars;
    let verbose = env_vars.get(TMS_VERBOSE).unwrap();
    let parse_response = env_vars.get(TMS_PARSE_RESPONSE).unwrap();

    // Issue the command.
    let goose_resp = user.get("v1/tms/version").await?;
    match goose_resp.response {
        Ok(r) => {
            if parse_response != "false" {
                match r.text().await 
                {
                    Ok(content) => {
                        if verbose != "false" {println!("*** Version: {}", content);}
                    },
                    Err(e) => {
                        return TransactionResult::Err(Box::new(TransactionError::Reqwest(e)));
                    }
                };
           }
        },
        Err(e) => {
            println!("*** Error: {}", e);
        }
    }

    Ok(())
}

// ******************************************************************************
//                             Private Utilities
// ******************************************************************************
// ------------------------------------------------------------------------------
// init_runtime_context:
// ------------------------------------------------------------------------------
pub fn init_runtime_context() -> RuntimeCtx {
    RuntimeCtx {env_vars: get_env_vars()}
}

// ------------------------------------------------------------------------------
// get_env_vars:
// ------------------------------------------------------------------------------
fn get_env_vars() -> HashMap<&'static str, String> {
    // Create the environment variable hashmap.
    let mut env_map = HashMap::new();

    // ----- X_TMS_TENANT
    let val = env::var(X_TMS_TENANT).unwrap_or_else(
                                |_| {"".to_string()});
    if !val.is_empty() {env_map.insert(X_TMS_TENANT, val);}

    // ----- X_TMS_CLIENT_ID
    let val = env::var(X_TMS_CLIENT_ID).unwrap_or_else(
                                |_| {"".to_string()});
    if !val.is_empty() {env_map.insert(X_TMS_CLIENT_ID, val);}

    // ----- X_TMS_CLIENT_SECRET
    let val = env::var(X_TMS_CLIENT_SECRET).unwrap_or_else(
                                |_| {"".to_string()});
    if !val.is_empty() {env_map.insert(X_TMS_CLIENT_SECRET, val);}

    // ----- X_TMS_ADMIN_ID
    let val = env::var(X_TMS_ADMIN_ID).unwrap_or_else(
                                |_| {"".to_string()});
    if !val.is_empty() {env_map.insert(X_TMS_ADMIN_ID, val);}

    // ----- X_TMS_ADMIN_SECRET
    let val = env::var(X_TMS_ADMIN_SECRET).unwrap_or_else(
                                |_| {"".to_string()});
    if !val.is_empty() {env_map.insert(X_TMS_ADMIN_SECRET, val);}

    // ----- TMS_VERBOSE
    // Set to the default "false" if not found; anything other than  
    // "false" will trigger printing.  This only takes effect if 
    // TMS_PARSE_RESPONSE = true.
    let val = env::var(TMS_VERBOSE).unwrap_or_else(
                                |_| {"false".to_string()});
    env_map.insert(TMS_VERBOSE, val);

    // ----- TMS_PARSE_RESPONSE
    // Set to the default "false" if not found; anything other than  
    // "false" will trigger the response to be parsed on receipt.
    let val = env::var(TMS_PARSE_RESPONSE).unwrap_or_else(
                                |_| {"false".to_string()});
    env_map.insert(TMS_PARSE_RESPONSE, val);

    // Always output the environment settings.
    // NOTE: Secrets are printed out!
    println!("\n-------------------------------------------");
    println!("TMS Environment Map: {:#?}", env_map);
    println!("-------------------------------------------\n");
    
    env_map
}
