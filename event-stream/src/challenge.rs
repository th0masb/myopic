use anyhow::Result;

use crate::config::{AppConfig, StringMatcher, TimeConstraints};
use crate::events::{Challenge, TimeControl};
use crate::lichess::LichessClient;

pub struct ChallengeService {
    client: LichessClient,
    validity_checks: Vec<Box<dyn ValidityCheck + Send + Sync>>,
}

impl ChallengeService {
    pub fn new(parameters: &AppConfig) -> ChallengeService {
        ChallengeService {
            client: LichessClient::new(parameters.lichess_bot.auth_token.clone()),
            validity_checks: vec![
                Box::new(parameters.time_constraints.clone()),
                Box::new(VariantCheck),
                Box::new(parameters.lichess_bot.user_matchers.clone()),
            ],
        }
    }

    pub async fn process_challenge(&self, challenge: Challenge) -> Result<String> {
        if self
            .validity_checks
            .iter()
            .all(|check| check.accepts(&challenge))
        {
            log::info!("Challenge is valid, posting accept response");
            self.client
                .post_challenge_response(&challenge, "accept")
                .await
                .map(|status| format!("{} from challenge accept", status))
        } else {
            log::info!("Illegal challenge: {:?}", &challenge);
            self.client
                .post_challenge_response(&challenge, "decline")
                .await
                .map(|status| format!("{} from challenge decline", status))
        }
    }
}

trait ValidityCheck {
    fn accepts(&self, challenge: &Challenge) -> bool;
}

const STANDARD_VARIANT_KEY: &'static str = "standard";
// TODO Support custom FEN variant
//const FEN_VARIANT_KEY: &'static str = "fromPosition";

struct VariantCheck;
impl ValidityCheck for VariantCheck {
    fn accepts(&self, challenge: &Challenge) -> bool {
        challenge.variant.key.as_str() == STANDARD_VARIANT_KEY
    }
}

impl ValidityCheck for TimeConstraints {
    fn accepts(&self, challenge: &Challenge) -> bool {
        match &challenge.time_control {
            TimeControl::Unlimited | TimeControl::Correspondence { .. } => false,
            TimeControl::Clock { clock } => {
                self.min_initial_time_secs <= clock.limit
                    && self.max_initial_time_secs >= clock.limit
                    && self.min_increment_secs <= clock.increment
                    && self.max_increment_secs >= clock.increment
            }
        }
    }
}

impl ValidityCheck for Vec<StringMatcher> {
    fn accepts(&self, challenge: &Challenge) -> bool {
        let name = challenge.challenger.name.as_str();
        self.iter()
            .find(|&matcher| matcher.pattern.is_match(name))
            .map(|matcher| matcher.include)
            .unwrap_or(false)
    }
}
