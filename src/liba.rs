use std::{borrow::{BorrowMut, Borrow}, ops::{Deref, DerefMut}, fmt::Debug};
pub mod stack;
use stack::*;

pub struct Object{
    data: Vec<u32>,
    current_state: Option<Box<dyn State>>,
}
impl Object{
    /// creates new empty object with defoult InitState.
    pub fn new()->Self{
        Self { 
            data: Vec::new(),
            current_state: Some(Box::new(InitState::new())), 
        }
    }

    /// show current state. State-specifsc behaviour
    pub fn state(&self)->String{
        self.current_state.as_ref().unwrap().name()
    }
    /// change current data. State-specific behaviour
    pub fn extend(&mut self, slise: &[u32]){
        if let Some(current_state) = self.current_state.take(){
            self.current_state = Some(current_state.extend(self, slise));
        }
    }
    /// trying to transit to another state. 
    /// returns new object with new state if transition is successfull,
    /// returns object with it`s input current state if transition is impossible.
    /// returns object with it`s previous state if transition is possible but something wrong.
    /// If previous state as absent - returns object with InitState.
    pub fn try_transit(mut self, target: Box<dyn State>)->Self{

        match target.try_transit(&mut self){

            Status::Ok(state) => {
                
                let current_state = self.take_state();

                if &current_state != &state{
                    Stack::push(current_state);
                }
               
                self.set_new_state(state)
            },

            Status::Unreachable => return self,

            Status::Fail => {
                let previous_state = Stack::pull().unwrap_or(Box::new(InitState::new()));
                self.set_new_state(previous_state)
            }
        };

        self
    }

    // private API
    fn set_new_state(&mut self, state: Box<dyn State>){
        self.current_state = Some(state);
    }
    fn take_state(&mut self)->Box<dyn State>{
        self.current_state.take().unwrap()
    }
}

impl Deref for Object{
    type Target = Vec<u32>;
    fn deref(&self) -> &Self::Target {
        self.data.borrow()
    }
}
impl DerefMut for Object {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data.borrow_mut()
    }
}






pub enum Status{
    Ok(Box<dyn State>),
    Unreachable,
    Fail
}

/// trait describes state-specific behavior
pub trait ExtendBehavior{
    fn name(&self)->String;
    fn extend(self: Box<Self>, obj: &mut Object, slise: &[u32])->Box<dyn State>;
}
/// trait describes how to do make a transition for each state.
pub trait State : ExtendBehavior + Debug{
    fn try_transit(&self, obj: &mut Object)->Status;
    fn id(&self)->u8; 
}

impl PartialEq<Box<dyn State>> for Box<dyn State>{
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}






// states
#[derive(Debug)]
pub struct InitState(u8);
impl InitState {
    pub fn new()->Self{
        Self(0)
    }
}
#[derive(Debug)]
pub struct StateA(u8);
impl StateA {
    pub fn new()->Self{
        Self(1)
    }
}
#[derive(Debug)]
pub struct StateB(u8);
impl StateB {
    pub fn new()->Self{
        Self(2)
    }
}




// states implementation

impl State for InitState{
    fn try_transit(&self, _obj: &mut Object)->Status {
        Status::Ok(Box::new(InitState::new()))
    }
    fn id(&self)->u8 {
        self.0
    }
}
impl State for StateA{
    fn try_transit(&self, obj: &mut Object)->Status {

        // condition exampl. StateA is unreachable for each objects that have data len == 0.
        if obj.len() == 0{
            return Status::Unreachable
        }

        // do some changes and conditions..

        Status::Ok(Box::new(StateA::new()))
    }
    fn id(&self)->u8 {
        self.0
    }
}
impl State for StateB{
    fn try_transit(&self, _obj: &mut Object)->Status {
        Status::Fail
    }
    fn id(&self)->u8 {
        self.0
    }
}




impl ExtendBehavior for InitState{
    fn name(&self)->String {
        format!("Init-State")
    }
    fn extend(self: Box<Self>, obj: &mut Object, slise: &[u32])->Box<dyn State>{
        **obj = Vec::from(slise);
        self
    }
}
impl ExtendBehavior for StateA{
    fn name(&self)->String {
        format!("State-A")
    }
    fn extend(self: Box<Self>, obj: &mut Object, slise: &[u32])->Box<dyn State>{
        obj.extend_from_slice(slise);
        self
    }
}
impl ExtendBehavior for StateB{
    fn name(&self)->String {
        format!("State-B")
    }
    fn extend(self: Box<Self>, obj: &mut Object, _slise: &[u32])->Box<dyn State>{
        obj.clear();
        self
    }
}
