
mod liba;
use liba::*;
use crate::liba::stack::Stack;


fn main() {

   // example:

   let mut object = Object::new();
   println!("state: {},    data: {:?}", object.state(), object.as_slice());

   object.extend(&[1,2,3,4,5,6,7,8,9,10]);
   object.extend(&[11, 12, 13, 14, 15, 16]);    // rewrite previous data

   println!("state: {},    data: {:?}", object.state(), object.as_slice());

   Stack::show();     // now transition history is clear

   // trying to transit InitState -> StateA
   let mut object = object.try_transit(Box::new(StateA::new()));

   Stack::show();    // here we can see previous state: InitState

   object.extend(&[11, 12, 13, 14, 15, 16]);    // extend previous data
   println!("state: {},    data: {:?}", object.state(), object.as_slice());


   // work with stack
   println!("\n");

   // transit from StateA to InitState. Always successfull.
   let object = object.try_transit(Box::new(InitState::new()));

   Stack::show();    // here we can see both previous state: InitState and StateA

   let object = object.try_transit(Box::new(InitState::new()));
   Stack::show();    // if state doesn`t change after transition executed - State doesn`t update

   let object = object.try_transit(Box::new(StateA::new()));
   Stack::show();    // here we can see three previous state

   let object = object.try_transit(Box::new(StateB::new()));   // always return Fail
   Stack::show();    // previous state from stack has pulled.

   println!("state: {},    data: {:?}", object.state(), object.as_slice());

}  
