use std::borrow::Borrow;
use std::fmt::Display;
use std::ops::Deref;
use std::sync::{Weak, Mutex};

use crate::object::{ExtendBehavior, State, Status, Object};  
use crate::object::BasicBehavior;
use std::sync::Arc;

use std::sync::MutexGuard;




#[derive(Debug)]
pub struct InitState<'a>(u8, Weak<Mutex<&'a mut dyn BasicBehavior>>);
impl <'a> InitState <'a>{
    pub fn new(obj: Weak<Mutex<&mut dyn BasicBehavior>>)->Self{
        Self(0, obj)
    }
}

impl <'a>State for InitState<'a>{
    fn try_transit(&self, obj_data: &mut dyn BasicBehavior)->Status {

        let arc_data = Arc::new(Mutex::new(obj_data));
        let weak_ref = Arc::downgrade(&arc_data);
        Status::Ok(Box::new(InitState::new(weak_ref)))
    }
    fn id(&self)->u8 {
        self.0
    }
}

impl<'a> Display for InitState<'a>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Init-State")
    }
}

impl<'a> ExtendBehavior for InitState<'a>{
    fn change_insurance(&mut self, receiver: &str){
        
    }
}

impl <'a> Deref for InitState<'a>{
    type Target = Arc<Mutex<dyn BasicBehavior>>;
    fn deref(&self) -> &Self::Target {
        &self.1.upgrade().unwrap()
    }
}