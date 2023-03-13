pub mod state_init;
pub mod state_sending;
mod delivered;

use std::fmt::Display;

use std::sync::{Arc, Mutex, Weak};

use crate::object::ExtendBehavior;
use super::BasicBehavior;


pub type BoxState = Box<dyn State>;

pub enum Status{
    Ok(Arc<Mutex<Box<dyn BasicBehavior>>>),
    Unreachable(Arc<Mutex<Box<dyn BasicBehavior>>>),
    Fail(Arc<Mutex<Box<dyn BasicBehavior>>>)
}

/// trait describes how to do make a transition for each state.
pub trait State : ExtendBehavior + Display{
    fn try_transit(&self, obj_data: Arc<Mutex<Box<dyn BasicBehavior>>>)->Status;
    fn id(&self)->u8; 
    fn set_data(&mut self, obj: Weak<Mutex<Box<dyn BasicBehavior>>>);
}

impl PartialEq<BoxState> for BoxState{
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}


