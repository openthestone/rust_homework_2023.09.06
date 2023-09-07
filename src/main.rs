#[macro_use]
extern crate std;

use std::collections::HashMap;
use std::alloc::*;
use std::ops::Deref;
use std::cell::RefCell;

// HashMap
macro_rules! hash_map {
    ($($key:expr => $val:expr),*) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert($key, $val);
            )*
            map
        }
    };
}

// MyRc
struct RefCnt<T> {
    count: i32,
    object: T
}
impl<T> RefCnt<T> {
    fn new(x: T) -> RefCnt<T> {
        RefCnt { count: 1, object: x }
    }
    fn change(&mut self, plus: i32) {
        self.count += plus;
    }
    fn strong_count(&self) -> i32 { self.count }
}
struct MyRc<T> {
    refcount: *mut RefCnt<T>,
}
impl<T> MyRc<T> {
    fn new(x: T) -> MyRc<T> {
        unsafe {
            let lt = Layout::new::<RefCnt<T>>();
            let tmp = alloc(lt);
            if tmp.is_null() {
                handle_alloc_error(lt);
            }
            *(tmp as *mut RefCnt<T>) = RefCnt::new(x);
            MyRc{ refcount: tmp as *mut RefCnt<T> }
        }
    }
    fn clone(&self) -> MyRc<T> {
        unsafe {
            (*(self.refcount as *mut RefCnt<T>)).change(1);
            MyRc { refcount: self.refcount as *mut RefCnt<T> }
        }
    }
    fn strong_count(&self) -> i32 {
        unsafe {
            (*(self.refcount as *mut RefCnt<T>)).strong_count()
        }
    }
}
impl<T> Deref for MyRc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe {
            &(*(self.refcount as *mut RefCnt<T>)).object
        }
    }
}

// SimpleStack
#[derive(Debug)]
struct SimpleStack<T> {
    stack: RefCell<Vec<T>>,
}

impl<T> SimpleStack<T> {
    fn new() -> SimpleStack<T> {
        SimpleStack {
            stack: RefCell::new(Vec::new()),
        }
    }
    fn push(&self, value: T) {
        self.stack.borrow_mut().push(value);
    }
    fn pop(&self) -> Option<T> {
        self.stack.borrow_mut().pop()
    }
}

fn main() {
    // HashMap
    let map = hash_map! {
        "one" => 1,
        "two" => 2,
        "three" => 3
    };
    println!("{:?}", map);

    // MyRc
    let rc1 = MyRc::new(5);
    println!("rc1: {:?}, count: {:?}", *rc1, MyRc::strong_count(&rc1));
    let rc2 = MyRc::clone(&rc1);
    let rc3 = MyRc::clone(&rc2);
    println!("rc1: {:?}, rc2: {:?}, rc3: {:?}, count: {:?}", *rc1, *rc2, *rc3, MyRc::strong_count(&rc1));

    // SimpleStack
    let stack = SimpleStack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("Popped value: {:?}", stack.pop());
    println!("Popped value: {:?}", stack.pop());
    stack.push(4);
    println!("Popped value: {:?}", stack.pop());
    println!("Popped value: {:?}", stack.pop());
    println!("Popped value: {:?}", stack.pop());
}
