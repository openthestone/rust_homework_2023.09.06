#[macro_use]
extern crate std;

use std::collections::HashMap;
use std::ops::Deref;
use std::cell::RefCell;

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

static mut CNT: i32 = 0;
struct MyRc<T>(T);
impl<T> MyRc<T> {
    fn new(x: T) -> MyRc<T> {
        unsafe {
            CNT += 1;
        }
        MyRc(x)
    }
    fn clone(x: &MyRc<T>) -> &MyRc<T> {
        unsafe {
            CNT += 1;
        }
        x
    }
    fn strong_count(_x: &MyRc<T>) -> i32 {
        unsafe {
            CNT
        }
    }
}
impl<T> Deref for MyRc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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
    let rc3 = MyRc::clone(rc2);
    println!("rc1: {:?}, rc2: {:?}, rc3: {:?}, count: {:?}", *rc1, **rc2, **rc3, MyRc::strong_count(&rc1));

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
