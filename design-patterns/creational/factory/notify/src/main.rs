#![allow(unused)]

use std::{io::Error, io::ErrorKind};

trait Notification {
    fn send(&self,message:&str);
}

enum NotificationType {
    Email(i32),
    SMS(i32),
}

struct EmailNotification {
    id:i32

}

impl EmailNotification {
    fn new(id:i32) -> Self {
        Self {
            id
        }
    }
}

impl Notification for EmailNotification {
    fn send(&self,message:&str) {
        // Email Sending Logic
        println!("Email {}: {message}", self.id);
    }
}

struct SMSNotification {
    id:i32

}

impl SMSNotification {
    fn new(id:i32) -> Self {
        Self {
            id
        }
    }
}

impl Notification for SMSNotification  {
    fn send(&self,message:&str) {
        // SMS Sending Logic
        println!("SMS {}: {message}", self.id);
    }
}

struct NotificationFactory {

}

impl NotificationFactory {
    fn create(notification_type:Option<NotificationType>) -> Result<Box<dyn Notification>,Error> {
        match notification_type {
            Some(NotificationType::SMS(id)) => {
                Ok(Box::new(SMSNotification::new(id)))
            }, 
            Some(NotificationType::Email(id)) => {
                Ok(Box::new(EmailNotification::new(id)))
            },
            None => {
                Err(Error::new(ErrorKind::InvalidInput,"error while creating notification object"))
            }
        }
    }
}



fn main() {
    let notification_factory = NotificationFactory::create(Some(NotificationType::Email(5)));
    match notification_factory {
        Ok(notification) => {
            notification.as_ref().send("hello");
        },
        Err(err) => {
            println!("{err}");
        }
    }
}
