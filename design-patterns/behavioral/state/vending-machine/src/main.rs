#![allow(unused)]
use std::rc::Rc;

trait VendingMachineState {
    fn insert_coin(&self,machine:&mut VendingMachine);
    fn select_product(&self,machine:&mut VendingMachine);
    fn dispense(&self,machine:&mut VendingMachine);
}

struct NoCoinState {

}

impl NoCoinState {
    fn new() -> Rc<dyn VendingMachineState> {
        Rc::new(
            Self{

            }
        )
    }
}

impl VendingMachineState for NoCoinState {
    fn insert_coin(&self,machine:&mut VendingMachine) {
        println!("coin inserted");
        machine.set_state(HasCoinState::new());
    }

    fn select_product(&self,machine:&mut VendingMachine) {
        println!("insert coin first");
    }

    fn dispense(&self,machine:&mut VendingMachine) {
        println!("insert coin first")
    }
}

struct HasCoinState {

}

impl HasCoinState {
    fn new() -> Rc<dyn VendingMachineState> {
        Rc::new(
            Self{
                
            }
        )
    }
}

impl VendingMachineState for HasCoinState {
    fn insert_coin(&self,machine:&mut VendingMachine) {
        println!("coin already inserted");
    }

    fn select_product(&self,machine:&mut VendingMachine) {
        println!("product selected");
        machine.set_state(DispenseState::new());
    }

    fn dispense(&self,machine:&mut VendingMachine) {
        println!("select product first");
    }
}

struct DispenseState {

}

impl DispenseState {
    fn new() -> Rc<dyn VendingMachineState> {
        Rc::new(
            Self{
                
            }
        )
    }
}

impl VendingMachineState for DispenseState {
    fn insert_coin(&self,machine:&mut VendingMachine) {
        println!("please wait,dispensing");
    }
    fn select_product(&self,machine:&mut VendingMachine) {
        println!("please wait,dispensing");
    }
    fn dispense(&self,machine:&mut VendingMachine) {
        println!("product dispensed");
        machine.set_state(NoCoinState::new());
    }
}



struct VendingMachine{
    current_state: Rc<dyn VendingMachineState>
}

impl VendingMachine {

    pub fn new() -> Self {
        Self {
            current_state: NoCoinState::new()
        }
    }

    pub fn insert_coin(&mut self) {
        self.current_state.clone().insert_coin(self);
    }
    pub fn select_product(&mut self) {
        self.current_state.clone().select_product(self);
    }
    pub fn dispense(&mut self) {
        self.current_state.clone().dispense(self);
    }

    fn set_state(&mut self, state:Rc<dyn VendingMachineState>) {
        self.current_state = state;
    }
}



fn main() {
    let mut machine = VendingMachine::new();
    machine.select_product();
    machine.insert_coin();
    machine.select_product();
    machine.dispense();
}
