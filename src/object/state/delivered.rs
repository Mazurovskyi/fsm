use crate::object::{ExtendBehavior, State, Status};  

use crate::object::BasicBehavior;
use std::fmt::Display;
use std::sync::{Arc, Mutex, Weak};

#[derive(Debug)]
pub struct Delivered(u8, Option<Weak<Mutex<Box<dyn BasicBehavior>>>>);
impl Delivered {
    pub fn new()->Self{
        Self(2, None)
    }
}


impl State for Delivered{
    fn try_transit(&self, obj_data: Arc<Mutex<Box<dyn BasicBehavior>>>)->Status {

        Status::Fail(obj_data)
    }
    fn id(&self)->u8 {
        self.0
    }
    fn set_data(&mut self, obj: Weak<Mutex<Box<dyn BasicBehavior>>>){
        self.1 = Some(obj)
    }
}

impl Display for Delivered{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "State-B")
    }
}

impl ExtendBehavior for Delivered{
    fn change_insurance(&mut self, _receiver: &str){
        println!("Tle mail has delivered! You cannot change the receiver.")
    }
}
