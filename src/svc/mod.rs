#![allow(unused)]

use std::collections::HashMap;
use std::rc::Rc;
use super::user::{User,UserTier};
use super::rate_limiter::{RateLimiter,RateLimiterType,RateLimiterFactory};

pub struct RateLimiterService {
    rate_limiters: HashMap<UserTier,Box<dyn RateLimiter>>
}

impl RateLimiterService {
    pub fn new() -> Self {
        let mut rate_limiters = HashMap::new();
        rate_limiters.insert(UserTier::FREE, RateLimiterFactory::new(10,60,RateLimiterType::TokenBucket));
        rate_limiters.insert(UserTier::PAID, RateLimiterFactory::new(100,60,RateLimiterType::TokenBucket));
        Self {
            rate_limiters
        }
    }

    pub fn allow_request(&mut self,user:Rc<User>) -> bool {
        let tier = user.clone().get_tier();
        self.rate_limiters.get_mut(&tier).unwrap().allow_request(user.clone().get_username())
    }
}