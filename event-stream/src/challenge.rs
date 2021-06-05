use anyhow::Result;

use crate::events::{Challenge, ClockTimeControl, TimeControl, Variant};
use crate::lichess::LichessClient;
use crate::params::ApplicationParameters;
use crate::validity::TimeValidity;

const STANDARD_VARIANT_KEY: &'static str = "standard";
const FEN_VARIANT_KEY: &'static str = "fromPosition";

pub struct ChallengeService {
    client: LichessClient,
    time_validity: TimeValidity,
}

impl ChallengeService {
    pub fn new(parameters: &ApplicationParameters) -> ChallengeService {
        ChallengeService {
            client: LichessClient::new(parameters.lichess_auth_token.clone()),
            time_validity: TimeValidity {
                initial_bounds_secs: (
                    parameters.min_initial_time_secs,
                    parameters.max_initial_time_secs,
                ),
                increment_bounds_secs: (
                    parameters.min_increment_secs,
                    parameters.max_increment_secs,
                ),
            },
        }
    }

    pub async fn process_challenge(&self, challenge: Challenge) -> Result<String> {
        match challenge.time_control {
            TimeControl::Unlimited | TimeControl::Correspondence { .. } => {
                log::info!("Cannot play game without real time clock...");
                self.client
                    .post_challenge_response(&challenge, "decline")
                    .await
                    .map(|status| format!("{} from challenge decline", status))
            }
            TimeControl::Clock { ref clock } => {
                if self.is_legal_challenge(clock, &challenge.variant) {
                    self.client
                        .post_challenge_response(&challenge, "accept")
                        .await
                        .map(|status| format!("{} from challenge accept", status))
                } else {
                    log::info!("Illegal challenge: {:?} {:?}", challenge.variant, clock);
                    self.client
                        .post_challenge_response(&challenge, "decline")
                        .await
                        .map(|status| format!("{} from challenge decline", status))
                }
            }
        }
    }

    fn is_legal_challenge(&self, time_control: &ClockTimeControl, variant: &Variant) -> bool {
        self.time_validity.is_valid(time_control) && variant.key.as_str() == STANDARD_VARIANT_KEY
            || variant.key.as_str() == FEN_VARIANT_KEY
    }
}
