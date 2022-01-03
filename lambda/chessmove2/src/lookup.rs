use async_trait::async_trait;
use myopic_brain::anyhow;
use myopic_brain::{ChessBoard, FenPart, Move};
use rusoto_core::Region;
use rusoto_dynamodb::DynamoDbClient;
use std::rc::Rc;
use std::sync::Arc;

#[async_trait]
pub trait LookupMoveService {
    async fn lookup(
        &self,
        position: Arc<dyn ChessBoard + Send + Sync>,
    ) -> anyhow::Result<Option<Move>>;
}

pub struct DynamoOpeningMoveService {
    table_name: String,
    table_region: Region,
    position_key: String,
    move_key: String,
    max_depth: u8,
    client: DynamoDbClient,
}

#[async_trait]
impl LookupMoveService for DynamoOpeningMoveService {
    async fn lookup(
        &self,
        position: Arc<dyn ChessBoard + Send + Sync>,
    ) -> anyhow::Result<Option<Move>> {
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
