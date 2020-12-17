use crate::game::LookupService;
use itertools::Itertools;
use myopic_brain::{parse, FenComponent, MutBoard};
use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, GetItemInput};
use serde::export::fmt::Debug;
use std::collections::HashMap;
use std::str::FromStr;

const MOVE_FREQ_SEPARATOR: &'static str = ":";

#[derive(Debug, Clone)]
pub struct DynamoDbOpeningServiceConfig {
    pub table_name: String,
    pub position_key: String,
    pub move_key: String,
    pub table_region: Region,
}

pub struct DynamoDbOpeningService {
    table_name: String,
    primary_key: String,
    recommended_move_attribute: String,
    client: DynamoDbClient,
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

impl LookupService for DynamoDbOpeningService {
    fn lookup_move(&self, uci_sequence: &str) -> Result<Option<String>, String> {
        let query_position = parse::position_from_uci(uci_sequence)?.to_partial_fen(&[
            FenComponent::Board,
            FenComponent::Active,
            FenComponent::CastlingRights,
        ]);
        log::info!(
            "Querying table {} for position {}",
            self.table_name,
            query_position
        );
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(self.client.get_item(self.create_request(query_position)?))
            .map_err(|err| format!("{}", err))
            .and_then(|response| match response.item {
                None => {
                    log::info!("No match found!");
                    Ok(None)
                }
                Some(attributes) => match attributes.get(&self.recommended_move_attribute) {
                    None => Err(format!(
                        "Position exists but missing recommended move attribute"
                    )),
                    Some(attribute) => match &attribute.ss {
                        None => Err(format!(
                            "Position and recommended move attribute exist but not string set type"
                        )),
                        Some(move_set) => match choose_move(move_set, rand::random) {
                            None => Err(format!("Position exists with no valid recommendations!")),
                            Some(mv) => {
                                log::info!("Found matching set {:?}!", move_set);
                                log::info!("Chose {} from set", &mv);
                                Ok(Some(mv))
                            }
                        },
                    },
                },
            })
    }
}

fn choose_move(available: &Vec<String>, f: impl Fn() -> usize) -> Option<String> {
    let records = available
        .into_iter()
        .filter_map(|s| MoveRecord::from_str(s.as_str()).ok())
        .sorted_by_key(|r| r.freq)
        .collect::<Vec<_>>();

    let frequency_sum = records.iter().map(|r| r.freq).sum::<usize>();

    if frequency_sum == 0 {
        None
    } else {
        let record_choice = f() % frequency_sum;
        let mut sum = 0usize;
        for record in records {
            if sum <= record_choice && record_choice < sum + record.freq {
                return Some(record.mv);
            }
            sum += record.freq;
        }
        None
    }
}

struct MoveRecord {
    mv: String,
    freq: usize,
}

impl FromStr for MoveRecord {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s
            .split(MOVE_FREQ_SEPARATOR)
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let err = format!("Cannot parse {} as MoveRecord", s);
        let cmp0 = split.get(0).ok_or(err.clone())?;
        let cmp1 = split.get(1).ok_or(err.clone())?;
        let freq = cmp1.parse::<usize>().map_err(|_| err)?;
        Ok(MoveRecord {
            mv: cmp0.clone(),
            freq,
        })
    }
}

#[cfg(test)]
mod test {
    use super::choose_move;

    #[test]
    fn test_choose_move() {
        let choices = vec![
            format!("a2a3:1"),
            format!("b2b4:1"),
            format!("g8f6:3"),
            format!("e1g1:20"),
        ];

        assert_eq!(Some(format!("a2a3")), choose_move(&choices, || { 0 }));
        assert_eq!(Some(format!("b2b4")), choose_move(&choices, || { 1 }));

        for i in 2..5 {
            assert_eq!(Some(format!("g8f6")), choose_move(&choices, || { i }));
        }

        for i in 5..25 {
            assert_eq!(Some(format!("e1g1")), choose_move(&choices, || { i }));
        }

        assert_eq!(Some(format!("a2a3")), choose_move(&choices, || { 25 }));
    }
}
