//! Simple program to validate data contract JSON schemas against DPP

use serde_json;
use std::{sync::Arc, collections::HashSet};
use dpp::{self, prelude::Identifier, Convertible, consensus::ConsensusError};

/// Paste a data contract here between r#" and "# then do `cargo run`;
pub const JSON_STRING: &str = r#"{"nft":{"type":"object","properties":{"b":{"enum":[1,2,3]},"a":{"type":"number"}},"dependencies":{"b":{"properties":{"a":{"minimum":4}}}},"additionalProperties":false}}"#;

fn validate(json_obj: serde_json::Value) -> Vec<String> {

    let protocol_version_validator = dpp::version::ProtocolVersionValidator::default();
    let data_contract_validator = dpp::data_contract::validation::data_contract_validator::DataContractValidator::new(Arc::new(protocol_version_validator));
    let factory = dpp::data_contract::DataContractFactory::new(1, Arc::new(data_contract_validator));
    let owner_id = Identifier::random();
    let contract_result = factory
        .create(owner_id, json_obj.clone().into(), None, None);

    match contract_result {
        Ok(contract) => {
            let results = contract.data_contract.validate(&contract.data_contract.to_cleaned_object().unwrap()).unwrap_or_default();
            let errors = results.errors;
            extract_basic_error_messages(&errors)
        },
        Err(e) => {
            let mut error_messages: Vec<String> = Vec::new();
            error_messages.push(format!("{}", e));
            error_messages
        }
    }
}

fn extract_basic_error_messages(errors: &[ConsensusError]) -> Vec<String> {
    let messages: Vec<String> = errors
        .iter()
        .filter_map(|error| {
            if let ConsensusError::BasicError(inner) = error {
                if let dpp::errors::consensus::basic::basic_error::BasicError::JsonSchemaError(json_error) = inner {
                    Some(format!("JsonSchemaError: {}, Path: {}", json_error.error_summary().to_string(), json_error.instance_path().to_string()))
                } else { 
                    Some(format!("{}", inner)) 
                }
            } else {
                None
            }
        })
        .collect();

    let messages: HashSet<String> = messages.into_iter().collect();
    let messages: Vec<String> = messages.into_iter().collect();

    messages
}

fn main() {
    let json_obj = serde_json::from_str(&JSON_STRING).expect("Failed to parse JSON_STRING");
    let validation_result = validate(json_obj);
    if validation_result.is_empty() {
        println!("The JSON passes validation.");
    } else {
        println!("Validation errors:");
        for message in validation_result {
            println!("{}", message)
        }
    }    
}
