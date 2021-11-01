//use std::borrow::Borrow;
//use std::borrow::Borrow;
use std::cell::RefCell;
//use std::cmp::Ordering;
use redblacktree::*;
use std::rc::Rc;
fn main() {
    let mut a = ATree::build();
    a = a.push(2);
    println!("{}", a.show());
    a = a.push(3);
    println!("{}", a.show());
    a = a.push(1);
    println!("{}", a.show());

    let mut f = Rc::new(RefCell::new(Tree::default()));
    for i in (1..10).rev() {
        f = f.push(i);
        println!("2 it is{}", f.show());
        println!("2 it is{}", f.show_color());
    }
    let mut f = Rc::new(RefCell::new(Tree::default()));
    //f.borrow_mut().num = -4;
    for i in -10..0 {
        f = f.push(i);
        println!("3 it is{}", f.show());
        println!("3 it is{}", f.show_color());
    }
    let d = f.search(-7);
    if d.is_some() {
        println!("{}", d.as_ref().unwrap().show());
    }
    //println!("{}", f.show());
}
