
use std::sync::Mutex;
use once_cell::sync::Lazy;

use crate::liba::*;

static mut STACK: Lazy<Mutex<Vec<Box<dyn State>>>> = Lazy::new(||Mutex::new(Vec::with_capacity(10)));

/// stores transition history.
pub (crate) struct Stack(Mutex<Vec<Box<dyn State>>>);

impl Stack {

    pub (crate) fn push(element: Box<dyn State>){
        unsafe{
            let mut guard = STACK.lock().unwrap();

            if guard.len() == guard.capacity(){
                let _el = guard.remove(0);
            }

            guard.push(element)
        }
    }

    pub (crate ) fn pull()->Result<Box<dyn State>, ()>{
        unsafe{
            STACK.lock().unwrap().pop().ok_or(())
        }
    }

    pub fn show(){
        unsafe{
            println!("STACK: {:?}", STACK.lock().unwrap().as_slice())
        }
    }
}
