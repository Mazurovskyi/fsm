use crate::object::{ExtendBehavior, State, Status};  
use crate::BasicBehavior;
use std::fmt::Display;

#[derive(Debug)]
pub struct StateSending(u8);
impl StateSending {
    pub fn new()->Box<Self>{
        Box::new(Self(1))
    }
}


impl State for StateSending{
    fn try_transit(&self, obj_data: &mut dyn BasicBehavior)->Status {

        if obj_data.receiver() == ""{
            return Status::Fail
        }

        // do some another changes and conditions..

        *obj_data.insurance_mut() = 120;

        Status::Ok(StateSending::new())
    }
    fn id(&self)->u8 {
        self.0
    }
}
impl Display for StateSending{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "State-Sending")
    }
}
impl ExtendBehavior for StateSending{
    fn change_insurance(&mut self, receiver: &str) {
        
    }
}