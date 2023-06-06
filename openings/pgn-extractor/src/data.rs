use std::collections::HashMap;

#[derive(Default)]
pub struct PositionStore {
    inner: HashMap<String, Vec<MoveRecord>>,
    stats: StoreStats,
}

#[derive(Default, Serialize)]
pub struct StoreStats {
    new_position_inserts: usize,
    new_alternate_move_inserts: usize,
    duplicates: usize,
}

#[derive(Clone, Serialize)]
pub struct MoveRecord {
    mv: String,
    freq: usize,
}

impl MoveRecord {
    fn new(mv: String) -> MoveRecord {
        MoveRecord { mv, freq: 1 }
    }
}

#[derive(Serialize)]
pub struct DatabaseEntry {
    pub position: String,
    pub moves: Vec<MoveRecord>,
}

impl PositionStore {
    pub fn stats(&self) -> &StoreStats {
        &self.stats
    }

    pub fn entries(&self) -> impl Iterator<Item = DatabaseEntry> + '_ {
        self.inner.iter().map(|(k, v)| DatabaseEntry {
            position: k.clone(),
            moves: v.iter().map(|r| r.clone()).collect(),
        })
    }

    pub fn process(&mut self, position: String, suggested_move: String) {
        match self.inner.get_mut(position.as_str()) {
            None => {
                self.stats.new_position_inserts += 1;
                self.inner.insert(position, vec![MoveRecord::new(suggested_move)]);
            }
            Some(records) => {
                match records.iter_mut().find(|record| record.mv == suggested_move) {
                    None => {
                        self.stats.new_alternate_move_inserts += 1;
                        records.push(MoveRecord::new(suggested_move))
                    }
                    Some(record) => {
                        self.stats.duplicates += 1;
                        record.freq += 1
                    }
                };
            }
        };
    }
}
