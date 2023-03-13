pub mod init;
pub mod sending;
mod delivered;

use std::fmt::Display;

use std::sync::{Arc, Mutex, Weak};

use crate::object::{ExtendBehavior,ObjectData};



pub type BoxState = Box<dyn State>;

pub enum Status{
    Ok(Arc<ObjectData>),
    Unreachable(Arc<ObjectData>),
    Fail(Arc<ObjectData>)
}

/// trait describes how to do make a transition for each state.
pub trait State : ExtendBehavior + Display{
    fn try_transit(&self, obj_data: Arc<ObjectData>)->Status;
    fn id(&self)->u8; 
    fn set_data(&mut self, obj: Weak<ObjectData>);
}

impl PartialEq<BoxState> for BoxState{
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}


