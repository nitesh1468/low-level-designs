#![allow(unused)]

use super::RateLimiter;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct SlidingWindowRateLimiter {
    requests: u32,
    window_in_sec: u32,
}

impl SlidingWindowRateLimiter {
    pub fn new(requests: u32, window_in_sec: u32) -> Self {
        Self {
            requests,
            window_in_sec,
        }
    }
}

impl RateLimiter for SlidingWindowRateLimiter {
    fn allow_request(&mut self, user_id: String) -> bool {
        false
    }
}
