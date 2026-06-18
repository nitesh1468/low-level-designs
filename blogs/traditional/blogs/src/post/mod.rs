#![allow(unused)]

use crate::state::Draft;

use super::state;

pub struct Post {
    state: Option<Box<dyn state::State>>,
    pub content: String
}

impl Post {
    pub fn new() -> Self {
        Self {
            state: Some(Box::new(state::Draft::new())),
            content: String::new()
        }
    }

    pub fn add_text(&mut self,text:&str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
    
}