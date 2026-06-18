#![allow(unused)]
use super::post;

pub trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self,post:&'a post::Post)-> &'a str;

}

pub struct Draft {

}

impl Draft {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview::new())
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self,post:&'a post::Post)-> &'a str {
        ""
    }

}

struct PendingReview {

}

impl PendingReview {
    fn new() -> Self {
        Self {

        }
    }
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published::new())
    }

    fn content<'a>(&self,post:&'a post::Post)-> &'a str {
        ""
    }
}

struct Published {

}

impl Published {
    fn new() -> Self {
        Self {

        }
    }
}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self:Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self,post:&'a post::Post)-> &'a str {
        &post.content
    }
}

