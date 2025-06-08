use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct MinuteCounter {
    map: Mutex<HashMap<u64, usize>>, // key = minute timestamp
}

impl MinuteCounter {
    fn current_minute(&self) -> u64 {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        now.as_secs() / 60
    }

    pub fn new() -> Self {
        Self {
            map: Mutex::new(HashMap::new()),
        }
    }

    pub fn record_event(&self) {
        let minute = self.current_minute();
        let mut map = self.map.lock().unwrap();
        *map.entry(minute).or_insert(0) += 1;
        Self::prune(&mut map, minute);
    }

    pub fn count_last_hour(&self) -> usize {
        let now_minute = self.current_minute();
        let mut map = self.map.lock().unwrap();
        Self::prune(&mut map, now_minute);
        map.values().sum()
    }

    fn prune(map: &mut HashMap<u64, usize>, current_minute: u64) {
        let threshold = current_minute - 60;
        map.retain(|&minute, _| minute > threshold);
    }
}
