use std::collections::HashMap;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, Result};
use rusoto_dynamodb::{AttributeValue, AttributeValueUpdate, DynamoDb, DynamoDbClient, GetItemInput, PutItemInput, UpdateItemInput};
use rusoto_dynamodb::QueryInput;

use crate::config::ChallengeTableConfig;

const SECS_IN_DAY: u64 = 24 * 60 * 60;

mod attribute_keys {
    pub const CHALLENGER: &str = "ChallengerID";
    pub const CHALLENGE: &str = "ChallengeID";
    pub const EPOCH_DAY: &str = "ChallengeDay";
    pub const EXPIRY: &str = "Expiry";
    pub const STARTED: &str = "GameStarted";
}

pub struct ChallengeTableClient {
    params: ChallengeTableConfig,
    client: DynamoDbClient,
}

#[derive(Clone, Debug)]
pub struct ChallengeTableEntry {
    pub challenger: String,
    pub challenge_id: String,
    pub challenge_epoch_day: u64,
    pub game_started: bool,
}

#[derive(Clone, Debug)]
pub enum FetchEntriesRequest {
    Challenger(String),
    EpochDay(u64),
}

impl ChallengeTableClient {
    pub async fn insert_entry(
        &self,
        challenger: String,
        challenge: String,
    ) -> Result<()> {
        let request = init(|r: &mut PutItemInput| {
            r.table_name = self.params.id.name.clone();
            r.item = init(|dest: &mut HashMap<String, AttributeValue>| {
                dest.insert(
                    attribute_keys::CHALLENGER.to_owned(),
                    init(|a: &mut AttributeValue| { a.s = Some(challenger) }),
                );
                dest.insert(
                    attribute_keys::CHALLENGE.to_owned(),
                    init(|a: &mut AttributeValue| { a.s = Some(challenge) }),
                );
                dest.insert(
                    attribute_keys::STARTED.to_owned(),
                    init(|a: &mut AttributeValue| { a.bool = Some(false) }),
                );
                dest.insert(
                    attribute_keys::EPOCH_DAY.to_owned(),
                    init(|a: &mut AttributeValue| { a.n = Some(epoch_day().to_string()) }),
                );
                dest.insert(
                    attribute_keys::EXPIRY.to_owned(),
                    init(|a: &mut AttributeValue| {
                        a.n = Some((epoch_secs() + 2 * SECS_IN_DAY).to_string())
                    }),
                );
            });
        });
        self.client.put_item(request)
            .await
            .map(|_| ())
            .map_err(|e| anyhow!(e))
    }

    pub async fn set_game_started(
        &self,
        challenger: String,
        challenge: String,
    ) -> Result<()> {
        let request = init(|r: &mut UpdateItemInput| {
            r.table_name = self.params.id.name.clone();
            r.key = init(|k: &mut HashMap<String, AttributeValue>| {
                k.insert(
                    attribute_keys::CHALLENGER.to_owned(),
                    init(|a: &mut AttributeValue| { a.s = Some(challenger) })
                );
                k.insert(
                    attribute_keys::CHALLENGE.to_owned(),
                    init(|a: &mut AttributeValue| { a.s = Some(challenge) })
                );
            });
            r.attribute_updates = Some(
                init(|dest: &mut HashMap<String, AttributeValueUpdate>| {
                    dest.insert(
                        attribute_keys::STARTED.to_owned(),
                        init(|update: &mut AttributeValueUpdate| {
                            update.action = Some("SET".to_owned());
                            update.value = Some(
                                init(|a: &mut AttributeValue| { a.bool = Some(true) })
                            )
                        })
                    );
                })
            )
        });
        self.client.update_item(request)
            .await
            .map(|_| ())
            .map_err(|e| anyhow!(e))
    }

    pub async fn fetch_entries(
        &self,
        request: FetchEntriesRequest,
    ) -> Result<Vec<ChallengeTableEntry>> {
        let mut dest = Vec::new();
        let mut last_evaluated_key = None;
        loop {
            let query = self.build_query(request.clone(), last_evaluated_key.clone());
            let output = self.client.query(query).await?;
            if let Some(items) = output.items {
                dest.extend(self.extract_attributes(&items)?);
            }
            last_evaluated_key = output.last_evaluated_key;
            if last_evaluated_key.is_none() {
                break
            }
        }
        Ok(dest)
    }

    fn extract_attributes(
        &self,
        attributes: &Vec<HashMap<String, AttributeValue>>,
    ) -> Result<Vec<ChallengeTableEntry>> {
        let mut dest = Vec::new();
        for attr in attributes {
            dest.push(
                ChallengeTableEntry {
                    challenger: extract_attribute(
                        attr,
                        attribute_keys::CHALLENGER,
                        |a| a.s.clone()
                    )?,
                    challenge_id: extract_attribute(
                        attr,
                        attribute_keys::CHALLENGE,
                        |a| a.s.clone()
                    )?,
                    game_started: extract_attribute(
                        attr,
                        attribute_keys::STARTED,
                        |a| a.bool
                    )?,
                    challenge_epoch_day: extract_attribute(
                        attr,
                        attribute_keys::EPOCH_DAY,
                        |a| a.n.clone()
                    ).and_then(|v| v.parse::<u64>().map_err(|e| anyhow!(e)))?,
                }
            )
        }
        Ok(dest)
    }

    fn build_query(
        &self,
        request: FetchEntriesRequest,
        last_evaluated_key: Option<HashMap<String, AttributeValue>>
    ) -> QueryInput {
        init(|query: &mut QueryInput| {
            let params = &self.params;
            query.table_name = params.id.name.clone();
            query.exclusive_start_key = last_evaluated_key;
            query.index_name = match request {
                FetchEntriesRequest::Challenger(_) => None,
                FetchEntriesRequest::EpochDay(_) => Some(params.challenge_day_index_name.clone())
            };
            query.key_condition_expression = match request {
                FetchEntriesRequest::Challenger(id) => {
                    Some(format!("{} = :{}", params.challenger_attribute, id))
                },
                FetchEntriesRequest::EpochDay(epoch_day) => {
                    Some(format!("{} = :{}", params.challenge_day_attribute, epoch_day))
                }
            };
        })
    }
}

fn init<T, F>(configure: F) -> T
    where
        T: Default,
        F: FnOnce(&mut T) -> ()
{
    let mut instance = Default::default();
    configure(&mut instance);
    instance
}

fn extract_attribute<T, F: Fn(&AttributeValue) -> Option<T>>(
    attributes: &HashMap<String, AttributeValue>,
    key: &str,
    extractor: F,
) -> Result<T> {
    extractor(&attributes[key])
        .ok_or(anyhow!("Attribute {} could not be extracted from {:?}", key, attributes))
}

fn epoch_secs() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn epoch_day() -> u64 {
    epoch_secs() / 60 / 60 / 24
}
