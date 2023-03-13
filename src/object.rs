use std::borrow::{BorrowMut, Borrow};

use std::fmt::Display;
use std::ops::{Deref, DerefMut};

pub mod stack;
pub mod state;

use stack::Stack;
use state::{state_init::InitState, State, Status, BoxState};


/// trait describes how to interact with object data
pub trait BasicBehavior : Display{
    fn receiver(&self)->&str;
    fn receiver_mut(&mut self)->&mut String;
    fn insurance_mut(&mut self)->&mut u32;
}

/// trait describes state-specific behavior
pub trait ExtendBehavior{
    fn change_insurance(&mut self, receiver: &str);
}

pub struct Object{
    data: Option<Box<dyn BasicBehavior>>,
    current_state: Option<BoxState>,
    history: Stack
}
impl Object{
    pub fn from(data: Box<dyn BasicBehavior>)->Self{
        Self{
            data: Some(data),
            current_state: Some(Box::new(InitState::new())), 
            history: Stack::default()
        }
    }
}
impl BaseObject for Object{
    fn data(&self)->&dyn BasicBehavior{
        self.data.borrow()
    }
    fn extract_data(&mut self)->&mut dyn BasicBehavior{
        self.data.take().unwrap()
    }
    fn history(&mut self)->&mut Stack{
        self.history.borrow_mut()
    }
    fn state_mut(&mut self)->&mut Option<BoxState>{
        self.current_state.borrow_mut()
    }
    fn state(&self)->&Option<BoxState> {
        self.current_state.borrow()
    }
}
impl History for Object{}
impl Transition for Object{}

impl Deref for Object{
    type Target = Box<dyn BasicBehavior>;    //<Object<T> as BaseObject>::Data;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl DerefMut for Object{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\nstate: {}\ndata: {}\nhistory: {}\n", 
        self.current_state.as_ref().unwrap(), 
        self.data(),
        self.history)
    }
}
impl ExtendBehavior for Object{
    fn change_insurance(&mut self, receiver: &str){
       //self.state().as_ref().unwrap().change_insurance(self.data_mut(), receiver);
    }
 }









 







/// base trait should be implemented for each object.
pub trait BaseObject{

    /// type describes the inital data of each object. 
    /// May be a collection, smart pointer or an another struct.
    //type Data;

    fn data(&self)->&dyn BasicBehavior;
    fn extract_data(&mut self)->&mut dyn BasicBehavior;
    fn history(&mut self)->&mut Stack;
    fn state_mut(&mut self)->&mut Option<BoxState>;
    fn state(&self)->&Option<BoxState>;
}


/// trait to store object transitions history.
pub trait History : BaseObject{
    fn push_to_history(&mut self, element: BoxState) {
        self.history().push(element)
    }
    fn pull_from_history(&mut self)-> Result<BoxState, ()> {
        self.history().pull()
    }
}

/// object representation.
pub trait Transition : History{
    /// trying to transit to another state. 
    /// returns new object with new state if transition is successfull,
    /// returns object with it`s input current state if transition is impossible.
    /// returns object with it`s previous state if transition is possible but something wrong.
    /// If previous state as absent - returns object with InitState.
    /// required 'State' trait implementation.
    fn try_transit(mut self, target: BoxState)->Self where Self:Sized{

        //let mut serialized = self.serialize();

        match target.try_transit(self.extract_data()){
            Status::Ok(state) => {
                let current_state = self.take_state();

                if &current_state != &state{
                    self.push_to_history(current_state)
                }
                
                self.set_new_state(state)
            },
            Status::Unreachable => return self,
            Status::Fail => {
                let previous_state = self.pull_from_history().unwrap_or(Box::new(InitState::new()));
                self.set_new_state(previous_state)
            }
        };

        self
    }

    // this should be private API
    fn set_new_state(&mut self, state: BoxState){
        *self.state_mut() = Some(state);
    }
    fn take_state(&mut self)->BoxState{
        self.state_mut().take().unwrap()
    }

}




