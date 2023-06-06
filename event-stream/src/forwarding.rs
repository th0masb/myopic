use std::convert::Infallible;

use warp::http::StatusCode;

use crate::lichess::LichessClient;

#[derive(Deserialize, Serialize)]
pub struct ChallengeRequest {
    rated: bool,
    #[serde(rename = "clock.limit")]
    clock_limit: usize,
    #[serde(rename = "clock.increment")]
    clock_increment: usize,
    #[serde(default)]
    color: ColourOption,
}

#[derive(Deserialize, Serialize)]
pub enum ColourOption {
    #[serde(rename = "random")]
    Random,
    #[serde(rename = "white")]
    White,
    #[serde(rename = "black")]
    Black,
}
impl Default for ColourOption {
    fn default() -> Self {
        ColourOption::Random
    }
}

pub async fn challenge(
    client: &LichessClient,
    user: String,
    params: ChallengeRequest,
) -> Result<impl warp::Reply, Infallible> {
    log::info!("Challenging {} with game params {}", user, serde_json::to_string(&params).unwrap());
    let forward_response = client.post_challenge(user.as_str(), &params).await;
    let response = match forward_response {
        Ok((code, body)) => {
            log::info!("Received Lichess response code:{}, body:{}", code, body);
            warp::reply::with_status(body, code)
        }
        Err(e) => {
            log::error!("Error trying to contact Lichess: {}", e);
            warp::reply::with_status(
                format!("{{\"error\"}}:\"{}\"", e),
                StatusCode::INTERNAL_SERVER_ERROR,
            )
        }
    };
    Ok(response)
}
