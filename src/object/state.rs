pub mod state_init;
pub mod state_sending;
mod delivered;

use std::{fmt::{Debug, Display}, io::{Write, Read}};
use std::collections::HashMap;

use crate::object::ExtendBehavior;
use super::{Transition, BaseObject, BasicBehavior};


pub type BoxState = Box<dyn State>;

pub enum Status{
    Ok(BoxState),
    Unreachable,
    Fail
}

/// trait describes how to do make a transition for each state.
pub trait State : ExtendBehavior + Display{
    fn try_transit(&self, obj_data: &mut dyn BasicBehavior)->Status;
    fn id(&self)->u8; 
}

impl PartialEq<BoxState> for BoxState{
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}


