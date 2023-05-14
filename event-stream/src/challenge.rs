use crate::challenge_table::ChallengeTableClient;
use anyhow::Result;

use crate::config::{AppConfig, StringMatcher, TimeConstraints};
use crate::events::{Challenge, TimeControl};
use crate::lichess::LichessClient;

pub struct ChallengeService {
    lichess: LichessClient,
    challenge_table: ChallengeTableClient,
    validity_checks: Vec<Box<dyn ValidityCheck + Send + Sync>>,
}

impl ChallengeService {
    pub fn new(parameters: &AppConfig) -> ChallengeService {
        ChallengeService {
            lichess: LichessClient::new(parameters.lichess_bot.auth_token.clone()),
            challenge_table: ChallengeTableClient::new(&parameters.challenge_table),
            validity_checks: vec![
                Box::new(parameters.time_constraints.clone()),
                Box::new(VariantCheck),
                Box::new(parameters.lichess_bot.user_matchers.clone()),
            ],
        }
    }

    pub async fn process_challenge(&self, challenge: Challenge) -> Result<String> {
        let passes_static_checks = self
            .validity_checks
            .iter()
            .all(|check| check.accepts(&challenge));

        if passes_static_checks && self.passes_table_checks(&challenge).await? {
            log::info!("Persisting challenge {}", challenge.id);
            self.challenge_table
                .insert_challenge(challenge.challenger.name.as_str(), challenge.id.as_str())
                .await?;
            self.post_challenge_response(&challenge, "accept").await
        } else {
            log::info!("Illegal challenge: {:?}", &challenge);
            self.post_challenge_response(&challenge, "decline").await
        }
    }

    async fn post_challenge_response(
        &self,
        challenge: &Challenge,
        decision: &str,
    ) -> Result<String> {
        log::info!(
            "Posting {} response for challenge {}",
            decision,
            challenge.id
        );
        self.lichess
            .post_challenge_response(&challenge, decision)
            .await
            .map(|status| format!("{} from challenge {}", status, decision))
    }

    async fn passes_table_checks(&self, challenge: &Challenge) -> Result<bool> {
        let all_challenges_today = self.challenge_table.fetch_challenges_today().await?;

        Ok(all_challenges_today.len() <= 50
            // Rate limit users per day
            && all_challenges_today
                .iter()
                .filter(|c| c.challenger == challenge.challenger.name)
                .count()
                < 6
            // Don't accept duplicated challenge ids
            && !all_challenges_today
                .iter()
                .any(|c| c.challenge_id == challenge.id))
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
