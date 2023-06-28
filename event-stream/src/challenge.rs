use crate::challenge_table::ChallengeTableClient;
use anyhow::Result;
use lichess_events::events::{Challenge, TimeControl};
use lichess_events::lichess::LichessClient;

use crate::config::{AppConfig, StringMatcher, TimeConstraints};

pub struct ChallengeService {
    lichess: LichessClient,
    challenge_table: ChallengeTableClient,
    validity_checks: Vec<Box<dyn ValidityCheck + Send + Sync>>,
    max_daily_challenges: usize,
    max_daily_user_challenges: usize,
    rate_limit_exclusions: Vec<String>,
    our_id: String,
}

impl ChallengeService {
    pub fn new(parameters: &AppConfig) -> ChallengeService {
        ChallengeService {
            lichess: LichessClient::new(parameters.lichess_bot.auth_token.clone()),
            challenge_table: ChallengeTableClient::new(&parameters.rate_limits.challenge_table),
            max_daily_challenges: parameters.rate_limits.max_daily_challenges,
            max_daily_user_challenges: parameters.rate_limits.max_daily_user_challenges,
            rate_limit_exclusions: parameters.rate_limits.excluded.clone(),
            our_id: parameters.lichess_bot.bot_id.to_lowercase(),
            validity_checks: vec![
                Box::new(parameters.time_constraints.clone()),
                Box::new(VariantCheck),
                Box::new(parameters.lichess_bot.user_matchers.clone()),
            ],
        }
    }

    pub async fn process_challenge(&self, challenge: Challenge) -> Result<String> {
        let challenge_id = challenge.id.as_str();
        log::info!("Processing challenge {}", challenge_id);
        if challenge.challenger.id == self.our_id {
            self.challenge_table.insert_challenge(self.our_id.as_str(), challenge_id).await?;
            Ok(format!("Added entry for our challenge {}", challenge_id))
        } else {
            let passes_static_checks =
                self.validity_checks.iter().all(|check| check.accepts(&challenge));

            if passes_static_checks && self.passes_rate_limit_checks(&challenge).await? {
                self.challenge_table
                    .insert_challenge(challenge.challenger.id.as_str(), challenge_id)
                    .await?;
                self.post_challenge_response(&challenge, "accept").await
            } else {
                log::info!("Illegal challenge: {:?}", &challenge);
                self.post_challenge_response(&challenge, "decline").await
            }
        }
    }

    async fn post_challenge_response(
        &self,
        challenge: &Challenge,
        decision: &str,
    ) -> Result<String> {
        log::info!("Posting {} response for challenge {}", decision, challenge.id);
        self.lichess
            .post_challenge_response(&challenge, decision)
            .await
            .map(|status| format!("{} from challenge {}", status, decision))
    }

    async fn passes_rate_limit_checks(&self, challenge: &Challenge) -> Result<bool> {
        let all_challenges_today = self.challenge_table.fetch_challenges_today().await?;
        let user_id = challenge.challenger.id.as_str();
        let user_challenges_today =
            all_challenges_today.iter().filter(|c| c.challenger == user_id).count();

        log::info!(
            "{} challenges today, {} from {}",
            all_challenges_today.len(),
            user_challenges_today,
            user_id
        );

        let is_excluded = self.rate_limit_exclusions.contains(&challenge.challenger.id);
        let under_rate_limit = all_challenges_today.len() < self.max_daily_challenges
            && user_challenges_today < self.max_daily_user_challenges;

        Ok((is_excluded || under_rate_limit)
            // Don't accept duplicated challenge ids
            && !all_challenges_today.iter().any(|c| c.challenge_id == challenge.id))
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
        let name = challenge.challenger.id.as_str();
        self.iter()
            .find(|&matcher| matcher.pattern.is_match(name))
            .map(|matcher| matcher.include)
            .unwrap_or(false)
    }
}
