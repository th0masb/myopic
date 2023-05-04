use std::collections::HashMap;

use anyhow::Result;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, GetItemInput};
use rusoto_dynamodb::QueryInput;

use crate::config::ChallengeTableConfig;

pub struct ChallengeTableClient {
    params: ChallengeTableConfig,
    client: DynamoDbClient,
}

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
        attributes: &Vec<HashMap<String, AttributeValue>>
    ) -> Result<Vec<ChallengeTableEntry>> {
        todo!()
    }

    fn build_query(
        &self,
        request: FetchEntriesRequest,
        last_evaluated_key: Option<HashMap<String, AttributeValue>>
    ) -> QueryInput {
        let mut query = QueryInput::default();
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
        query
    }
}