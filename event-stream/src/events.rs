#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum LichessEvent {
    #[serde(rename = "gameStart")]
    GameStart { game: GameStart },

    #[serde(rename = "challenge")]
    Challenge { challenge: Challenge },
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Challenge {
    pub id: String,
    pub variant: Variant,
    #[serde(rename = "timeControl")]
    pub time_control: TimeControl,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum TimeControl {
    #[serde(rename = "unlimited")]
    Unlimited,
    #[serde(rename = "correspondence")]
    Correspondence {
        #[serde(rename = "daysPerTurn")]
        days_per_turn: u32,
    },
    #[serde(rename = "clock")]
    Clock {
        #[serde(flatten)]
        clock: ClockTimeControl,
    },
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ClockTimeControl {
    pub limit: u32,
    pub increment: u32,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Variant {
    pub key: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GameStart {
    pub id: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;

    #[test]
    fn deserialize_game_start() {
        let json = r#"
        {
          "type": "gameStart",
          "game": {
            "id": "1lsvP62l"
          }
        }"#;

        match serde_json::from_str::<LichessEvent>(json) {
            Err(error) => panic!(format!("Parse error: {}", error)),
            Ok(event) => match event {
                LichessEvent::Challenge { .. } => panic!(format!("Wrong event: {:?}", event)),
                LichessEvent::GameStart { game } => {
                    assert_eq!(GameStart { id: "1lsvP62l".to_owned() }, game)
                }
            },
        }
    }

    #[test]
    fn deserialize_challenge_with_unlimited_time_control() {
        let json = r#"
        {
          "type": "challenge",
          "challenge": {
            "id": "x0ORBDis",
            "url": "https://lichess.org/x0ORBDis",
            "status": "created",
            "challenger": {
              "id": "th0masb",
              "name": "th0masb",
              "title": null,
              "rating": 1500,
              "provisional": true,
              "online": true
            },
            "destUser": {
              "id": "myopic-bot",
              "name": "myopic-bot",
              "title": "BOT",
              "rating": 1500,
              "provisional": true,
              "online": true
            },
            "variant": {
              "key": "standard",
              "name": "Standard",
              "short": "Std"
            },
            "rated": true,
            "speed": "correspondence",
            "timeControl": {
              "type": "unlimited"
            },
            "color": "random",
            "perf": {
              "icon": ";",
              "name": "Correspondence"
            }
          }
        }
        "#;

        match serde_json::from_str::<LichessEvent>(json) {
            Err(error) => panic!(format!("Parse error: {}", error)),
            Ok(event) => match event {
                LichessEvent::GameStart { .. } => panic!(format!("Wrong event: {:?}", event)),
                LichessEvent::Challenge { challenge } => assert_eq!(
                    Challenge {
                        id: "x0ORBDis".to_owned(),
                        variant: Variant { key: "standard".to_owned() },
                        time_control: TimeControl::Unlimited
                    },
                    challenge
                ),
            },
        }
    }

    #[test]
    fn deserialize_challenge_with_correspondence_time_control() {
        let json = r#"
        {
          "type": "challenge",
          "challenge": {
            "id": "qG23jvtf",
            "url": "https://lichess.org/qG23jvtf",
            "status": "created",
            "challenger": {
              "id": "th0masb",
              "name": "th0masb",
              "title": null,
              "rating": 1500,
              "provisional": true,
              "online": true
            },
            "destUser": {
              "id": "myopic-bot",
              "name": "myopic-bot",
              "title": "BOT",
              "rating": 1500,
              "provisional": true,
              "online": true
            },
            "variant": {
              "key": "standard",
              "name": "Standard",
              "short": "Std"
            },
            "rated": true,
            "speed": "correspondence",
            "timeControl": {
              "type": "correspondence",
              "daysPerTurn": 2
            },
            "color": "random",
            "perf": {
              "icon": ";",
              "name": "Correspondence"
            }
          }
        }
        "#;

        match serde_json::from_str::<LichessEvent>(json) {
            Err(error) => panic!(format!("Parse error: {}", error)),
            Ok(event) => match event {
                LichessEvent::GameStart { .. } => panic!(format!("Wrong event: {:?}", event)),
                LichessEvent::Challenge { challenge } => assert_eq!(
                    Challenge {
                        id: "qG23jvtf".to_owned(),
                        variant: Variant { key: "standard".to_owned() },
                        time_control: TimeControl::Correspondence { days_per_turn: 2 }
                    },
                    challenge
                ),
            },
        }
    }

    #[test]
    fn deserialize_challenge_with_clock_time_control() {
        let json = r#"
        {
          "type": "challenge",
          "challenge": {
            "id": "fLIBOP1V",
            "url": "https://lichess.org/fLIBOP1V",
            "status": "created",
            "challenger": {
              "id": "th0masb",
              "name": "th0masb",
              "title": null,
              "rating": 1841,
              "provisional": true,
              "online": true
            },
            "destUser": {
              "id": "myopic-bot",
              "name": "myopic-bot",
              "title": "BOT",
              "rating": 1500,
              "provisional": true,
              "online": true
            },
            "variant": {
              "key": "standard",
              "name": "Standard",
              "short": "Std"
            },
            "rated": true,
            "speed": "rapid",
            "timeControl": {
              "type": "clock",
              "limit": 600,
              "increment": 3,
              "show": "10+3"
            },
            "color": "random",
            "perf": {
              "icon": "",
              "name": "Rapid"
            }
          }
        }
        "#;

        match serde_json::from_str::<LichessEvent>(json) {
            Err(error) => panic!(format!("Parse error: {}", error)),
            Ok(event) => match event {
                LichessEvent::GameStart { .. } => panic!(format!("Wrong event: {:?}", event)),
                LichessEvent::Challenge { challenge } => assert_eq!(
                    Challenge {
                        id: "fLIBOP1V".to_owned(),
                        variant: Variant { key: "standard".to_owned() },
                        time_control: TimeControl::Clock {
                            clock: ClockTimeControl { limit: 600, increment: 3 }
                        }
                    },
                    challenge
                ),
            },
        }
    }
}
