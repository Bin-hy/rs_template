use std::cell::{Cell, RefCell};
use std::rc::Rc;

#[test]
fn rc_test() {
    let a = Rc::new(String::from("hello, world"));
    let b = Rc::clone(&a);
    println!("{} -> {}", Rc::strong_count(&a), Rc::strong_count(&b));
}

#[test]
fn cell_test() {
    let c = Cell::new("abcd");
    let one = c.get();
    c.set("efgh");
    let two = c.get();
    print!("{} -> {}", one, two);
}

fn rc_refcell_test() {
    let a = Rc::new(RefCell::new("hello, world".to_string()));
    let b = a.clone();
    let c = a.clone();
    c.borrow_mut().push_str("hi");
    println!("a:{}, b:{}, c:{}", a, b, c);
}
