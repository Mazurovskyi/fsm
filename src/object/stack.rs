
use std::ops::{Deref, DerefMut};
use std::borrow::{Borrow, BorrowMut};
use std::sync::{Mutex, Arc};
use std::fmt::{self, Formatter, Display};


use crate::object::state::BoxState;

type Pusher =  Box<dyn FnMut(BoxState) -> Result<(), BoxState>>;
type Puller =  Box<dyn FnMut() -> Result<BoxState, ()>>;
type Printer = Box<dyn Fn()->String>;


pub struct Stack{
    pusher:  Pusher,
    puller:  Puller,
    printer: Printer
}
impl Stack{
    pub fn push(&mut self, element: BoxState){
        let _res = (self.pusher)(element);
    }
    pub fn pull(&mut self)-> Result<BoxState, ()>{
        (self.puller)()
    }
}
impl Default for Stack{
    
    fn default() -> Self {
        let stack: Arc<Mutex<Vec<BoxState>>> = 
            Arc::new(Mutex::new(Vec::with_capacity(10)));

        let stack_pusher = Arc::clone(&stack);
        let stack_puller = Arc::clone(&stack);

    
        let pusher = move |element: BoxState|->Result<(), BoxState>{
            let mut stack_guard = stack_pusher.lock().unwrap();

            if let Err(element) = stack_guard.push_within_capacity(element){
                stack_guard.remove(0);
                return stack_guard.push_within_capacity(element)
            }

            Ok(())
    
        };

        let puller = move ||->Result<BoxState, ()>{
            stack_puller.lock().unwrap().pop().ok_or(())
        };

        let printer = move ||->String{
            stack.lock().unwrap().iter().map(|el|el.to_string()).collect()
        };

        Self{
            pusher: Box::new(pusher),
            puller: Box::new(puller),
            printer: Box::new(printer)
        }
    }
}


impl Display for Stack{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", (self.printer)())
    }
}