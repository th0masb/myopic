use std::env;
use std::error::Error;

/// Keys for required environment variables
const MYOPIC_AUTH_TOKEN: &'static str = "MYOPIC_AUTH_TOKEN";
const MYOPIC_REGION: &'static str = "MYOPIC_REGION";
const MYOPIC_FUNCTION_NAME: &'static str = "MYOPIC_FUNCTION_NAME";
const MYOPIC_BOT_ID: &'static str = "MYOPIC_BOT_ID";
const MYOPIC_EXPECTED_HALF_MOVES: &'static str = "MYOPIC_EXPECTED_HALF_MOVES";
const MYOPIC_MIN_INITIAL_TIME_SECS: &'static str = "MYOPIC_MIN_INITIAL_TIME_SECS";
const MYOPIC_MAX_INITIAL_TIME_SECS: &'static str = "MYOPIC_MAX_INITIAL_TIME_SECS";
const MYOPIC_MIN_INCREMENT_SECS: &'static str = "MYOPIC_MIN_INCREMENT_SECS";
const MYOPIC_MAX_INCREMENT_SECS: &'static str = "MYOPIC_MAX_INCREMENT_SECS";
const MYOPIC_MAX_LAMBDA_DURATION_MINS: &'static str = "MYOPIC_MAX_LAMBDA_DURATION_MINS";
const MYOPIC_INCREMENT_ALLOWANCE_MINS: &'static str = "MYOPIC_INCREMENT_ALLOWANCE_MINS";

#[derive(Debug, Clone)]
pub struct ApplicationParameters {
    pub auth_token: String,
    pub region: String,
    pub bot_id: String,
    pub function_name: String,
    pub expected_half_moves: u32,
    pub min_initial_time_secs: u32,
    pub max_initial_time_secs: u32,
    pub min_increment_secs: u32,
    pub max_increment_secs: u32,
    pub max_lambda_duration_mins: u8,
    pub increment_allowance_mins: u8,
}

impl ApplicationParameters {
    pub fn load() -> Result<ApplicationParameters, Box<dyn Error>> {
        Ok(ApplicationParameters {
            auth_token: env::var(MYOPIC_AUTH_TOKEN)?,
            expected_half_moves: env::var(MYOPIC_EXPECTED_HALF_MOVES)?.parse()?,
            bot_id: env::var(MYOPIC_BOT_ID)?,
            function_name: env::var(MYOPIC_FUNCTION_NAME)?,
            region: env::var(MYOPIC_REGION)?,
            min_initial_time_secs: env::var(MYOPIC_MIN_INITIAL_TIME_SECS)?.parse()?,
            max_initial_time_secs: env::var(MYOPIC_MAX_INITIAL_TIME_SECS)?.parse()?,
            min_increment_secs: env::var(MYOPIC_MIN_INCREMENT_SECS)?.parse()?,
            max_increment_secs: env::var(MYOPIC_MAX_INCREMENT_SECS)?.parse()?,
            max_lambda_duration_mins: env::var(MYOPIC_MAX_LAMBDA_DURATION_MINS)?.parse()?,
            increment_allowance_mins: env::var(MYOPIC_INCREMENT_ALLOWANCE_MINS)?.parse()?,
        })
    }

    pub fn to_lambda_invocation_payload(
        &self,
        game_id: String,
        depth: u8,
    ) -> Result<String, Box<dyn Error>> {
        serde_json::to_string(&PlayGameEvent {
            depth,
            game_id,
            region: self.region.clone(),
            auth_token: self.auth_token.clone(),
            function_name: self.function_name.clone(),
            bot_id: self.bot_id.clone(),
            expected_half_moves: self.expected_half_moves,
        })
        .map_err(|error| Box::new(error) as Box<dyn Error>)
    }
}

/// The input payload of this lambda
#[derive(Serialize, Deserialize, Clone)]
struct PlayGameEvent {
    /// The current call depth of the lambda invocation
    depth: u8,
    /// The region this lambda is deployed in
    region: String,
    /// The name of this lambda function
    #[serde(rename = "functionName")]
    function_name: String,
    /// The lichess game id this lambda will participate in
    #[serde(rename = "gameId")]
    game_id: String,
    /// An auth token for the lichess bot this lambda will play as
    #[serde(rename = "authToken")]
    auth_token: String,
    /// The id of the lichess bot this lambda will play as
    #[serde(rename = "botId")]
    bot_id: String,
    /// How many half moves we expect the game to last for
    #[serde(rename = "expectedHalfMoves")]
    expected_half_moves: u32,
}
