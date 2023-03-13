#![feature(vec_push_within_capacity)]

mod object;


use object::Object;
use crate::object::BasicBehavior;
use crate::object::BaseObject;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::fmt::Display;
use std::str;
use std::borrow::Borrow;
use crate::object::{Transition, ExtendBehavior};

use crate::object::state::state_sending::StateSending;

fn main() {

   // custom user struct
   let mail = Mail::new("Bilbo Beggins".to_string());

   // creating object instance in defoult InitState
   let object = Object::from(Box::new(mail));
   println!("{}", object);

   // trying transit to SendingState. Reciver is empty, so we`ll stay at current state
   let mut object = object.try_transit(StateSending::new());
   println!("{}", object);

   *object.receiver_mut() = "Frodo Beggins".to_string();
   
   // now we will transit
   let object = object.try_transit(StateSending::new());
   println!("{}", object);


}  


struct Mail{
   sender: String,
   receiver: String,
   insurance: u32
}
impl Mail{
   fn new(sender: String)->Self{
      Self { 
         sender,
         receiver: String::new(), 
         insurance: 0
      }
   }
}

impl BasicBehavior for Mail{
   fn receiver(&self)->&str{
      self.receiver.borrow()
   }
   fn receiver_mut(&mut self)->&mut String{
      self.receiver.borrow_mut()
   }
   fn insurance_mut(&mut self)->&mut u32 {
      self.insurance.borrow_mut()
   }
}


impl Display for Mail{
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "mail [sender: {}     receiver: {}      insurance: {}]",self.sender, self.receiver, self.insurance)
   }
}