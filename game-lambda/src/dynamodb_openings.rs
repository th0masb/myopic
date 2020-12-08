use crate::game::OpeningService;
use myopic_board::{parse, MutBoard};
use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, GetItemInput};
use serde::export::fmt::Debug;
use std::collections::HashMap;

pub struct DynamoDbOpeningService {
    table_name: String,
    primary_key: String,
    recommended_move_attribute: String,
    client: DynamoDbClient,
}

#[derive(Debug, Clone)]
pub struct DynamoDbOpeningServiceConfig {
    pub table_name: String,
    pub position_key: String,
    pub move_key: String,
    pub table_region: Region,
}

impl OpeningService for DynamoDbOpeningService {
    fn get_recommended_move(&self, uci_sequence: &str) -> Result<Option<String>, String> {
        let query_position = parse::position_from_uci(uci_sequence)?.to_timeless_fen();
        log::info!("Querying table {} for position {}", self.table_name, query_position);
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(self.client.get_item(self.create_request(query_position)?))
            .map_err(|err| format!("{}", err))
            .and_then(|response| match response.item {
                None => {
                    log::info!("No match found!");
                    Ok(None)
                },
                Some(attributes) => match attributes.get(&self.recommended_move_attribute) {
                    None => Err(format!("Position exists but missing recommended move attribute")),
                    Some(attribute) => match &attribute.ss {
                        None => Err(format!(
                            "Position and recommended move attribute exist but not string set type"
                        )),
                        Some(move_set) => match choose_move(move_set) {
                            None => Err(format!("Position exists with no recommendations!")),
                            Some(mv) => {
                                log::info!("Found matching set {:?}!", move_set);
                                Ok(Some(mv))
                            },
                        },
                    },
                },
            })
    }
}

impl DynamoDbOpeningService {
    pub fn new(config: DynamoDbOpeningServiceConfig) -> DynamoDbOpeningService {
        DynamoDbOpeningService {
            table_name: config.table_name,
            primary_key: config.position_key,
            recommended_move_attribute: config.move_key,
            client: DynamoDbClient::new(config.table_region),
        }
    }

    fn create_request(&self, query_position: String) -> Result<GetItemInput, String> {
        // Create key
        let mut av = AttributeValue::default();
        av.s = Some(query_position);
        let mut key = HashMap::new();
        key.insert(self.primary_key.clone(), av);
        // Create request
        let mut request = GetItemInput::default();
        request.table_name = self.table_name.clone();
        request.key = key;
        Ok(request)
    }
}

fn choose_move(available: &Vec<String>) -> Option<String> {
    if available.is_empty() {
        None
    } else {
        Some(available[rand::random::<usize>() % available.len()].clone())
    }
}
