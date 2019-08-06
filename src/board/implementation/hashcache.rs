const CACHE_SIZE: usize = 20;

/// The hashcache is a circular fixed sized array which tracks a sequence
/// of board hashings. When a new hash is added to the head of the sequence
/// the tail hash is lost. When a hash is popped from the head of the
/// sequence a replacement tail hash must be provided. The cache can check
/// for three repetitions of the same hash value which would imply a drawn
///  (by repetition) game.
#[derive(Debug, Clone, PartialOrd, PartialEq, Eq)]
pub struct HashCache {
    /// Records how many pop operations required to return to initial state.
    pop_dist: usize,
    /// Fixed size array which maintains the hash values.
    cache: [u64; CACHE_SIZE],
}

impl HashCache {
    /// Create a new cache at a given point in the game with a supplied
    /// position hash.
    pub fn new(position_hash: u64, n_previous_positions: usize) -> HashCache {
        let pop_dist = n_previous_positions;
        let cache = [064; CACHE_SIZE];
        let mut dest = HashCache { pop_dist: 0, cache };
        for i in 0..pop_dist {
            dest.push_head(i as u64);
        }
        dest.cache[pop_dist % CACHE_SIZE] = position_hash;
        dest
    }

    fn head_index(&self) -> usize {
        self.pop_dist % CACHE_SIZE
    }

    fn tail_index(&self) -> usize {
        (self.pop_dist + 1) % CACHE_SIZE
    }

    pub fn head(&self) -> u64 {
        self.cache[self.head_index()]
    }

    pub fn tail(&self) -> u64 {
        self.cache[self.tail_index()]
    }

    pub fn position_count(&self) -> usize {
        self.pop_dist + 1
    }

    pub fn push_head(&mut self, new_head: u64) {
        self.pop_dist += 1;
        let index = self.head_index();
        self.cache[index] = new_head;
    }

    pub fn pop_head(&mut self, new_tail: u64) {
        debug_assert!(self.pop_dist > 0);
        let index = self.head_index();
        self.cache[index] = new_tail;
        self.pop_dist -= 1;
    }

    pub fn has_three_repetitions(&self) -> bool {
        if self.pop_dist < CACHE_SIZE {
            false
        } else {
            let mut cache = self.cache.clone();
            cache.sort();
            let mut count = 1;
            let mut last = cache[0];
            for &hash in cache.into_iter().skip(1) {
                if hash == last {
                    count += 1;
                    if count == 3 {
                        break;
                    }
                } else {
                    count = 1;
                    last = hash;
                }
            }
            count == 3
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn n_consecutive(n: usize) -> HashCache {
        let mut cache = HashCache::new(0u64, 0);
        for n in 1..n {
            cache.push_head(n as u64);
        }
        cache
    }

    #[test]
    fn test_push_pop_head() {
        let (cs, n) = (CACHE_SIZE as u64, (2 * CACHE_SIZE) as u64);
        let init_cache = n_consecutive(n as usize);
        let mut cache = init_cache.clone();

        assert_eq!(n - cs, cache.tail(), "{:?}", cache);
        cache.push_head(n);
        assert_eq!(n - cs + 1, cache.tail(), "{:?}", cache);
        cache.push_head(n);
        assert_eq!(n - cs + 2, cache.tail(), "{:?}", cache);
        cache.push_head(n);
        assert_eq!(n - cs + 3, cache.tail(), "{:?}", cache);
        cache.push_head(n);

        // Put the results back
        cache.pop_head(n - cs + 3);
        cache.pop_head(n - cs + 2);
        cache.pop_head(n - cs + 1);
        cache.pop_head(n - cs);
        assert_eq!(init_cache, cache);
    }

    #[test]
    fn test_three_repetitions() {
        let cs = CACHE_SIZE;
        // Change test values if cache size changes
        assert_eq!(20, cs);
        // Test return false if not enough elements
        let mut cache1 = n_consecutive(cs - 5);
        cache1.push_head(2u64);
        cache1.push_head(5u64);
        cache1.push_head(2u64);
        assert!(!cache1.has_three_repetitions());

        // Test return false if enough elements but not three reps
        let mut cache2 = n_consecutive(cs);
        cache2.push_head(18u64);
        cache2.push_head(5u64);
        cache2.push_head(17u64);
        assert!(!cache2.has_three_repetitions());

        // Test return true if enough elements and three reps
        let mut cache3 = n_consecutive(cs);
        cache3.push_head(15u64);
        cache3.push_head(12u64);
        cache3.push_head(15u64);
        assert!(cache3.has_three_repetitions());
    }
}
