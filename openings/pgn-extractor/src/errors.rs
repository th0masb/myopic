use std::collections::HashMap;
use serde_derive::Serialize;

#[derive(Serialize, Default)]
pub struct Errors {
    read_error_total: usize,
    read_error_locations: HashMap<String, Vec<usize>>,
    parse_error_total: usize,
    parse_error_locations: HashMap<String, Vec<usize>>,
}

impl Errors {
    pub fn add_read_error(&mut self, file: String, game_index: usize) {
        self.read_error_total += 1;
        Errors::add_location(&mut self.read_error_locations, file, game_index)
    }

    pub fn add_parse_error(&mut self, file: String, game_index: usize) {
        self.parse_error_total += 1;
        Errors::add_location(&mut self.parse_error_locations, file, game_index)
    }

    fn add_location(map: &mut HashMap<String, Vec<usize>>, file: String, game_index: usize) {
        match map.get_mut(&file) {
            None => {
                map.insert(file, vec![game_index]);
            }
            Some(entries) => {
                entries.push(game_index);
            }
        }
    }
}
