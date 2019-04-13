use std::iter;

const CACHE_SIZE: usize = 20;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct HashCache {
    count: usize,
    cache: Vec<u64>,
}



impl HashCache {
    pub fn new() -> HashCache {
        HashCache {count: 0, cache: iter::repeat(0u64).take(CACHE_SIZE).collect()}
    }

    fn cache_index(&self) -> usize {
        self.count % CACHE_SIZE
    }

    pub fn push_head(&mut self, new_head: u64) -> u64 {
        self.count += 1;
        let index = self.cache_index();
        let old_tail = self.cache[index];
        self.cache[index] = new_head;
        old_tail
    }

    pub fn pop_head(&mut self, new_tail: u64) {
        debug_assert!(self.count > 0);
        let index = self.cache_index();
        self.cache[index] = new_tail;
        self.count -= 1;
    }

    pub fn head(&self) -> u64 {
        self.cache[self.cache_index()]
    }

    pub fn has_three_repetitions(&self) -> bool {
        if self.count < CACHE_SIZE {
            false
        } else {
            let mut cache = self.cache.clone();
            cache.sort();
            let mut count = 1;
            let mut last = cache[0];
            for hash in cache.into_iter().skip(1) {
               if hash == last {
                   count += 1;
                   if count == 3 {
                       break;
                   }
               }  else {
                   count = 1;
                   last = hash;
               }
            }
            count == 3
        }
    }
}
