#![allow(unused)]

mod rate_limiter;
mod user;
mod svc;

use svc::RateLimiterService;

use crate::user::{User,UserTier};
use std::rc::Rc;

fn main() {
    let mut svc = RateLimiterService::new();
    let free_user = Rc::new(User::new("user1", UserTier::FREE));
    let paid_user = Rc::new(User::new("user2", UserTier::PAID));

    println!("############## Free User's Requests ###############");
    for idx in 1..=2000 {
        let status = svc.allow_request(free_user.clone());
        if !status {
            continue;
        }
        println!("request {idx} for free user is {}",status);
    }

    // println!("############## Paid User's Requests ###############");
    for idx in 1..=2000 {
        let status = svc.allow_request(paid_user.clone());
        if !status {
            continue;
        }
        println!("request {idx} for paid user is {}",status);
    }

    

    
}
