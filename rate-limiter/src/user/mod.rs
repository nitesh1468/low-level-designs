#![allow(unused)]

#[derive(PartialEq,Eq,Hash,Clone)]
pub enum UserTier {
    FREE,
    PAID,
}

pub struct User {
    id: String,
    tier: UserTier,
}

impl User {
    pub fn new(id: &str, tier: UserTier) -> Self {
        Self {
            id: id.to_owned(),
            tier,
        }
    }

    pub fn get_username(&self) -> String {
        self.id.clone()
    }

    pub fn get_tier(&self) -> UserTier {
        self.tier.clone()
    }
}
