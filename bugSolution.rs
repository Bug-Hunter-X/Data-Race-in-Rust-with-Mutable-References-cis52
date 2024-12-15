use std::cell::RefCell;

fn main() {    let x = RefCell::new(5);    {
        let mut y = x.borrow_mut();
        *y = 6;
    }
    {
        let mut z = x.borrow_mut();
        *z = 7;
    }
    println!("x = {}", x.borrow());}   