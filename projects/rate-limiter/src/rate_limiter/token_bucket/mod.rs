#![allow(unused)]

use super::RateLimiter;
use std::collections::HashMap;
use dashmap::DashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct TokenBucketRateLimiter {
    tokens: Arc<DashMap<String, u32>>,
    last_refill_time: HashMap<String, u64>,
    requests: u32,
    window_in_sec: u32,
}

impl TokenBucketRateLimiter {
    pub fn new(requests: u32, window_in_sec: u32) -> Self {
        Self {
            requests,
            window_in_sec,
            last_refill_time: HashMap::new(),
            tokens: Arc::new(DashMap::new()),
        }
    }

    fn refill(&mut self, user_id: &str) -> u32 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time error")
            .as_secs();
        if self.last_refill_time.get(user_id) == None {
            self.last_refill_time.insert(user_id.to_owned(), now);
            return self.requests;
        }

        let &last_refil_time = self.last_refill_time.get(user_id).unwrap();
        let refill_tokens =
            ((now - last_refil_time) as f64) * (self.requests as f64) / (self.window_in_sec as f64);
        if refill_tokens > 0. {
            self.last_refill_time.insert(user_id.to_owned(), now);
        }
        return refill_tokens as u32;
    }
}

impl RateLimiter for TokenBucketRateLimiter {
    fn allow_request(&mut self, user_id: String) -> bool {
        let refill_tokens = self.refill(&user_id);
        if self.tokens.clone().get(&user_id).is_none() {
            self.tokens.clone().insert(user_id.clone(), self.requests);
        }
        let tokens = self.tokens.clone();
        let mut current_token = tokens.get_mut(&user_id).unwrap();
        *current_token = std::cmp::min(self.requests, *current_token + refill_tokens);
        if *current_token > 0 {
            *current_token -= 1;
            return true;
        }
        false
    }
}
