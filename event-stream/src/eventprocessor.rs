use crate::challenge::ChallengeService;
use crate::events::LichessEvent;
use crate::gamestart::GameStartService;
use crate::userstatus::StatusService;
use response_stream::LoopAction;

pub struct EventProcessor {
    pub challenge_service: ChallengeService,
    pub gamestart_service: GameStartService,
    pub status_service: StatusService,
}

impl EventProcessor {
    pub async fn handle_stream_read(&mut self, line: &str) -> LoopAction<()> {
        if line.is_empty() {
            self.user_status().await
        } else {
            match serde_json::from_str::<LichessEvent>(line) {
                Err(parse_error) => log::warn!("Parse error: {}", parse_error),
                Ok(event) => {
                    log::info!("Received event: {}", line);
                    self.handle_event(event).await
                }
            };
            LoopAction::Continue
        }
    }

    async fn user_status(&mut self) -> LoopAction<()> {
        match self.status_service.user_status().await {
            Err(e) => {
                log::warn!("Error fetching user status: {}", e);
                LoopAction::Continue
            }
            Ok(None) => LoopAction::Continue,
            Ok(Some(status)) => {
                if status.online {
                    LoopAction::Continue
                } else {
                    LoopAction::Break(())
                }
            }
        }
    }

    async fn handle_event(&self, event: LichessEvent) {
        match event {
            LichessEvent::Challenge { challenge } => {
                match self.challenge_service.process_challenge(challenge).await {
                    Ok(message) => log::info!("Processed challenge with message: {}", message),
                    Err(error) => log::warn!("Error processing challenge: {}", error),
                }
            }
            LichessEvent::GameStart { game } => {
                match self.gamestart_service.process_event(game).await {
                    Ok(message) => log::info!("Processed gamestart with message: {}", message),
                    Err(error) => log::warn!("Error processing gamestart: {}", error),
                }
            }
        }
    }
}
