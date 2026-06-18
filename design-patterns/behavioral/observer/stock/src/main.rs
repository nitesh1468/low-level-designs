#![allow(unused)]

use std::cell::RefCell;
use std::rc::Rc;

trait Observer {
    fn update(&self,symbol:&str,price:f64);
}

struct PriceDisplay { 
}

impl PriceDisplay {
    fn new() -> Rc<RefCell<Self>> {
        Rc::new(
            RefCell::new(
                Self {

                }
            )
        )
    }
}

impl Observer for PriceDisplay {
    fn update(&self,symbol:&str,price:f64) {
        println!("Display updated: {symbol} = ${price}");
    }
}

struct PriceAlert {
    threshold: f64
}

impl PriceAlert {
    fn new(threshold:f64) -> Rc<RefCell<Self>> {
        Rc::new(
            RefCell::new(
                Self {
                    threshold
                }
            )
        )
    }
}

impl Observer for PriceAlert {
    fn update(&self,symbol:&str,price:f64) {
        if price >= self.threshold  {
            println!("Alert! {symbol} exceeded ${}", self.threshold);
        }
    }
}



trait Subject {
    fn attach(&mut self,observer:Rc<RefCell<dyn Observer>>);
    fn detach(&mut self,observer:Rc<RefCell<dyn Observer>>);
    fn notify_observers(&self);
}

struct Stock {
    observers:Vec<Rc<RefCell<dyn Observer>>>,
    symbol:String,
    price:f64
}

impl Subject for Stock {
    fn attach(&mut self,observer:Rc<RefCell<dyn Observer>>) {
        self.observers.push(observer);
    }

    fn detach(&mut self,target_observer:Rc<RefCell<dyn Observer>>) {
        self.observers.retain(|observer|{
            !Rc::ptr_eq(observer, &target_observer)
        });
    }

    fn notify_observers(&self) {
        for observer in &self.observers {
            observer.borrow().update(&self.symbol, self.price);
        }
    }
}

impl Stock {
    fn new(symbol:&str,price:f64) -> Self {
        Self {
            observers: vec![],
            symbol: symbol.to_owned(),
            price
        }
    }

    fn set_price(&mut self,price:f64) {
        self.price = price;
        self.notify_observers();
    }
}
 

fn main() {
    let mut stock = Stock::new("AAPL", 130.00);
    let display = PriceDisplay::new();
    let alert = PriceAlert::new(150.00);
    stock.attach(display);
    stock.attach(alert);
    stock.set_price(145.);
    stock.set_price(155.);
}
