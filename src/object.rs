use std::borrow::{BorrowMut, Borrow};

use std::fmt::Display;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

pub mod stack;
pub mod state;

use stack::Stack;
use state::{init::InitState, State, Status, BoxState};


pub type ObjectData = Mutex<Box<dyn BasicBehavior>>;

/// trait describes how to interact with object data
pub trait BasicBehavior : Display{
    fn receiver(&self)->&str;
    fn receiver_mut(&mut self)->&mut String;
    fn insurance_mut(&mut self)->&mut u32;
}

/// trait describes state-specific behavior
pub trait ExtendBehavior{
    fn change_receiver(&mut self, receiver: &str);
}

pub struct Object{
    data: Option<Arc<ObjectData>>,
    current_state: Option<BoxState>,
    history: Stack
}
impl Object{
    pub fn from(data: Box<dyn BasicBehavior>)->Self{

        let obj_data = Arc::new(Mutex::new(data));
        let mut init_state = InitState::new();

        init_state.set_data(Arc::downgrade(&obj_data));

        Self{
            data: Some(obj_data),
            current_state: Some(Box::new(init_state)), 
            history: Stack::default()
        }
    }
}
impl BaseObject for Object{
    fn data(&self)->&Option<Arc<ObjectData>>{
        self.data.borrow()
    }
    fn mut_data(&mut self)->&mut Option<Arc<ObjectData>>{
        self.data.borrow_mut()
    }
    fn extract_data(&mut self)->Arc<ObjectData>{
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
    type Target = Arc<ObjectData>;    //<Object<T> as BaseObject>::Data;
    fn deref(&self) -> &Self::Target {
        self.data().as_ref().unwrap()
    }
}
impl DerefMut for Object{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.mut_data().as_mut().unwrap()
    }
}
impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\nstate: {}\ndata: {}\nhistory: {}\n", 
        self.current_state.as_ref().unwrap(), 
        self.data().as_ref().unwrap().lock().unwrap(),
        self.history)
    }
}
impl ExtendBehavior for Object{
    fn change_receiver(&mut self, receiver: &str){
       self.state_mut().as_mut().unwrap().change_receiver(receiver)
    }
 }









 







/// base trait should be implemented for each object.
pub trait BaseObject{

    /// type describes the inital data of each object. 
    /// May be a collection, smart pointer or an another struct.
    //type Data;

    fn data(&self)->&Option<Arc<ObjectData>>;
    fn mut_data(&mut self)->&mut Option<Arc<ObjectData>>;
    fn extract_data(&mut self)->Arc<ObjectData>;
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

        let obj_data = match target.try_transit(self.extract_data()){
            Status::Ok(obj_data) => {
                let current_state = self.take_state();

                if &current_state != &target{
                    self.push_to_history(current_state)
                }
                
                
                self.set_new_state(target);

                obj_data
            },
            Status::Unreachable(obj_data) => obj_data,
            Status::Fail(obj_data) => {

                let init_state = InitState::new();

                let previous_state = self.pull_from_history().unwrap_or(Box::new(init_state));
                self.set_new_state(previous_state);

                obj_data
            }
        };

        let weak_ref = Arc::downgrade(&obj_data);

        self.set_data(obj_data);

        self.state_mut().as_mut().unwrap().set_data(weak_ref);

        self
    }

    // this should be private API
    fn set_new_state(&mut self, state: BoxState){
        *self.state_mut() = Some(state);
    }
    fn take_state(&mut self)->BoxState{
        self.state_mut().take().unwrap()
    }
    fn set_data(&mut self, obj_data: Arc<ObjectData>){
        *self.mut_data() = Some(obj_data)
    }

}




