use crate::object::{ExtendBehavior, State, Status};  

use crate::object::BasicBehavior;
use std::fmt::Display;

#[derive(Debug)]
pub struct Delivered(u8);
impl Delivered {
    pub fn new()->Self{
        Self(2)
    }
}


impl State for Delivered{
    fn try_transit(&self, obj_data: &mut dyn BasicBehavior)->Status {
        Status::Fail
    }
    fn id(&self)->u8 {
        self.0
    }
}

impl Display for Delivered{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "State-B")
    }
}

impl ExtendBehavior for Delivered{
    fn change_insurance(&mut self, receiver: &str){
        println!("Tle mail has delivered! You cannot change the receiver.")
    }
}
