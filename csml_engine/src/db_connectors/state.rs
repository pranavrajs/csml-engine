#[cfg(feature = "dynamo")]
use crate::db_connectors::{dynamodb as dynamodb_connector, is_dynamodb};
#[cfg(feature = "mongo")]
use crate::db_connectors::{is_mongodb, mongodb as mongodb_connector};
use crate::error_messages::ERROR_DB_SETUP;
use crate::{ConversationInfo, Database, EngineError};
use csml_interpreter::data::Client;

pub fn delete_state_key(
    client: &Client,
    _type: &str,
    _key: &str,
    db: &mut Database,
) -> Result<(), EngineError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        return mongodb_connector::state::delete_state_key(client, _type, _key, db);
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        let db = dynamodb_connector::get_db(db)?;
        return dynamodb_connector::state::delete_state_key(client, _type, _key, db);
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn get_state_key(
    client: &Client,
    _type: &str,
    _key: &str,
    db: &mut Database,
) -> Result<Option<serde_json::Value>, EngineError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        let db = mongodb_connector::get_db(db)?;
        return mongodb_connector::state::get_state_key(client, _type, _key, db);
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        let db = dynamodb_connector::get_db(db)?;
        return dynamodb_connector::state::get_state_key(client, _type, _key, db);
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}

pub fn set_state_items(
    data: &mut ConversationInfo,
    _type: &str,
    keys_values: Vec<(&str, &serde_json::Value)>,
) -> Result<(), EngineError> {
    #[cfg(feature = "mongo")]
    if is_mongodb() {
        return mongodb_connector::state::set_state_items(data, _type, keys_values);
    }

    #[cfg(feature = "dynamo")]
    if is_dynamodb() {
        return dynamodb_connector::state::set_state_items(data, _type, keys_values);
    }

    Err(EngineError::Manager(ERROR_DB_SETUP.to_owned()))
}
