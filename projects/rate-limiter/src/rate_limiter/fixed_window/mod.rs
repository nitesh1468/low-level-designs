#![allow(unused)]

use super::RateLimiter;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct FixedWindowRateLimiter {
    requests: u32,
    window_in_sec: u32,
}

impl FixedWindowRateLimiter {
    pub fn new(requests: u32, window_in_sec: u32) -> Self {
        Self {
            requests,
            window_in_sec,
        }
    }
}

impl RateLimiter for FixedWindowRateLimiter {
    fn allow_request(&mut self, user_id: String) -> bool {
        false
    }
}
