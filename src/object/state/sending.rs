use crate::object::{ExtendBehavior, State, Status, ObjectData};  
use std::fmt::Display;
use std::sync::{Arc, Mutex, Weak};

#[derive(Debug)]
pub struct StateSending(u8, Option<Weak<ObjectData>>);
impl StateSending {
    pub fn new()->Box<Self>{
        Box::new(Self(1, None))
    }
}


impl State for StateSending{
    fn try_transit(&self, obj_data: Arc<ObjectData>)->Status {
        let mut obj_guard = obj_data.lock().unwrap();

        if obj_guard.receiver() == ""{
            drop(obj_guard);
            return Status::Fail(obj_data)
        }

        // do some another changes and conditions..

        *obj_guard.insurance_mut() = 120;

        drop(obj_guard);
        //let weak_ref = Arc::downgrade(&obj_data);
        Status::Ok(obj_data)
    }
    fn id(&self)->u8 {
        self.0
    }
    fn set_data(&mut self, obj: Weak<ObjectData>) {
        self.1 = Some(obj)
    }
}
impl Display for StateSending{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "State-Sending")
    }
}
impl ExtendBehavior for StateSending{
    fn change_receiver(&mut self, _receiver: &str) {
        println!("You cannot change the receiver while message sending.")
    }
}