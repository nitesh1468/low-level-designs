#![allow(unused)]

pub mod token_bucket;
pub mod fixed_window;
pub mod sliding_window;

use sliding_window::SlidingWindowRateLimiter;
use fixed_window::FixedWindowRateLimiter;
use token_bucket::TokenBucketRateLimiter;

pub trait RateLimiter {
    fn allow_request(&mut self, user_id: String) -> bool;
}

#[derive(Eq,PartialEq)]
pub enum RateLimiterType {
    TokenBucket,
    FixedWindow,
    SlidingWindow
}

pub struct RateLimiterFactory {
    
}

impl RateLimiterFactory {
    pub fn new(requests:u32, window_in_sec:u32,rate_limiter_type: RateLimiterType) -> Box<dyn RateLimiter> {
        match rate_limiter_type {
            RateLimiterType::TokenBucket => {
                Box::new(TokenBucketRateLimiter::new(requests, window_in_sec))
            },
            RateLimiterType::FixedWindow => {
                Box::new(FixedWindowRateLimiter::new(requests,window_in_sec))
            },
            RateLimiterType::SlidingWindow => {
                Box::new(SlidingWindowRateLimiter::new(requests,window_in_sec))
                
            }
        }
    }
}
