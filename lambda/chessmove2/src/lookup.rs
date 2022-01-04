use async_trait::async_trait;
use myopic_brain::anyhow;
use myopic_brain::{ChessBoard, FenPart, Move};
use rusoto_core::Region;
use rusoto_dynamodb::DynamoDbClient;
use std::rc::Rc;
use std::sync::Arc;
use lambda_payloads::chessmove2::OpeningTable;

#[async_trait]
pub trait LookupMoveService<B: ChessBoard + Send> {
    async fn lookup(&self, position: B) -> anyhow::Result<Option<Move>>;
}

pub struct DynamoOpeningMoveService {
    params: OpeningTable,
    client: DynamoDbClient,
}

impl From<OpeningTable> for DynamoOpeningMoveService {
    fn from(source: OpeningTable) -> Self {
        todo!()
    }
}

#[async_trait]
impl<B: ChessBoard + Send> LookupMoveService<B> for DynamoOpeningMoveService {
    async fn lookup(&self, position: B) -> anyhow::Result<Option<Move>> {
        let pos_count = position.position_count();
        if pos_count < self.max_depth as usize {
            log::info!(
                "Skipping lookup as {} exceeds max depth of {}",
                pos_count,
                self.max_depth
            );
            Ok(None)
        } else {
            let fen =
                position.to_fen_parts(&[FenPart::Board, FenPart::Active, FenPart::CastlingRights]);
            log::info!("Querying table {} for position {}", self.table_name, fen);
            todo!()
        }
    }
}
