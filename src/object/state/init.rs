use std::borrow::Borrow;
use std::fmt::Display;
use std::ops::Deref;
use std::sync::{Weak, Mutex};

use crate::object::{ExtendBehavior, State, Status};  
use crate::object::BasicBehavior;
use std::sync::Arc;






#[derive(Debug)]
pub struct InitState(u8, Option<Weak<Mutex<Box<dyn BasicBehavior>>>>);
impl InitState{
    pub fn new()->Self{
        Self(0, None)
    }
}

impl State for InitState{
    fn try_transit(&self, obj_data: Arc<Mutex<Box<dyn BasicBehavior>>>)->Status {
        Status::Ok(obj_data)
    }
    fn id(&self)->u8 {
        self.0
    }
    fn set_data(&mut self, obj: Weak<Mutex<Box<dyn BasicBehavior>>>) {
        self.1 = Some(obj)
    }
}

impl Display for InitState{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Init-State")
    }
}

impl ExtendBehavior for InitState{
    fn change_receiver(&mut self, receiver: &str){
        let binding = self.as_ref().unwrap().upgrade();
        let mut data_guard = binding.as_ref().unwrap().lock().unwrap();

        *data_guard.receiver_mut() = receiver.to_string();
        println!("Receiver changed!")
    }
}

impl Deref for InitState{
    type Target = Option<Weak<Mutex<Box<dyn BasicBehavior>>>>;
    fn deref(&self) -> &Self::Target {
        self.1.borrow()
    }
}