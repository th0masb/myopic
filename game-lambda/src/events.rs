use serde_derive;
use serde;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum GameEvent {
    #[serde(rename = "gameFull")]
    GameFull {
        #[serde(flatten)]
        content: GameFull
    },
    #[serde(rename = "gameState")]
    State {
        #[serde(flatten)]
        content: GameState
    }
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct GameFull {
    pub white: Player,
    pub black: Player,
    pub clock: Clock,
    pub state: GameState,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct GameState {
    pub moves: String,
    pub wtime: u64,
    pub btime: u64,
    pub winc: u64,
    pub binc: u64,
    pub wdraw: bool,
    pub bdraw: bool,
    pub status: String,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub title: Option<String>,
    pub rating: usize,
    pub provisional: bool,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Clock {
    pub initial: u64,
    pub increment: u64,
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;

    #[test]
    fn deserialize_state() {
        let json = r#"{
            "type": "gameState",
            "moves": "e2e4 c7c5",
            "wtime": 1000,
            "btime": 1000,
            "winc": 0,
            "binc": 0,
            "wdraw": false,
            "bdraw": false,
            "status": "started",
            "other": "value"
        }"#;

        match serde_json::from_str::<GameEvent>(json) {
            Err(error) => panic!(format!("Parse error {:?}", error)),
            Ok(event) => match event {
                GameEvent::GameFull { .. }  => panic!(format!("Wrong event {:?}", event)),
                GameEvent::State { content: state } => assert_eq!(
                    GameState {
                        moves: String::from("e2e4 c7c5"),
                        wtime: 1000,
                        btime: 1000,
                        winc: 0,
                        binc: 0,
                        wdraw: false,
                        bdraw: false,
                        status: String::from("started")
                    }, state)
            }
        }
    }

    #[test]
    fn deserialize_game_full() {
        let json = r#"{
            "type": "gameFull",
            "id": "123",
            "other": "value",
            "white": {
                "id": "th0masb",
                "name": "th0masb",
                "title": null,
                "rating": 1500,
                "provisional": true,
                "other": "value"
            },
            "black": {
                "id": "myopic-bot",
                "name": "myopic-bot",
                "title": "BOT",
                "rating": 1500,
                "provisional": true
            },
            "clock": {
                "initial": 1200000,
                "increment": 10000
            },
            "state": {
                "moves": "e2e4 e7e5",
                "wtime": 1000,
                "btime": 1000,
                "winc": 0,
                "binc": 0,
                "wdraw": false,
                "bdraw": false,
                "status": "started"
            }
        }"#;

        match serde_json::from_str::<GameEvent>(json) {
            Err(error) => panic!(format!("Parse error {:?}", error)),
            Ok(event) => match event {
                GameEvent::State { .. } => panic!(format!("Wrong type {:?}", event)),
                GameEvent::GameFull { content } => {
                    assert_eq!(Player {
                        id: String::from("th0masb"),
                        name: String::from("th0masb"),
                        title: None,
                        rating: 1500,
                        provisional: true
                    }, content.white);
                    assert_eq!(Player {
                        id: String::from("myopic-bot"),
                        name: String::from("myopic-bot"),
                        title: Some(String::from("BOT")),
                        rating: 1500,
                        provisional: true
                    }, content.black);
                    assert_eq!(Clock {
                        initial: 1200000,
                        increment: 10000,
                    }, content.clock);
                    assert_eq!(GameState {
                        moves: String::from("e2e4 e7e5"),
                        wtime: 1000,
                        btime: 1000,
                        winc: 0,
                        binc: 0,
                        wdraw: false,
                        bdraw: false,
                        status: String::from("started")
                    }, content.state);
                }
            }
        }
    }
}
