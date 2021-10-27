use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;
#[derive(Clone, Copy)]
enum Color {
    Red,
    Black,
}
// root is black, it is which red black tree required
impl Default for Color {
    fn default() -> Self {
        Self::Black
    }
}
#[derive(Default)]
struct Tree {
    num: i32,
    color: Color,
    parent: Option<Rc<RefCell<Tree>>>,
    left: Option<Rc<RefCell<Tree>>>,
    right: Option<Rc<RefCell<Tree>>>,
}
impl Tree {
    #[inline]
    fn new(num: i32, parent: Option<Rc<RefCell<Tree>>>, color: Color) -> Self {
        Self {
            num,
            color,
            parent,
            left: None,
            right: None,
        }
    }
}
type ATree = Rc<RefCell<Tree>>;
trait Insert {
    fn build(num: i32, parent: Option<Rc<RefCell<Tree>>>, color: Color) -> Self;
    fn insert(&self, input: i32);
}
trait Find {
    fn search(&self, input: i32) -> Option<ATree>;
}
trait Show {
    fn show(&self) -> String;
}
impl Insert for ATree {
    #[inline]
    fn build(num: i32, parent: Option<Rc<RefCell<Tree>>>, color: Color) -> Self {
        Rc::new(RefCell::new(Tree::new(num, parent, color)))
    }
    fn insert(&self, input: i32) {
        let num = self.borrow().num;
        if num > input {
            if self.borrow().left.is_none() {
                let color = self.borrow().color;
                match color {
                    Color::Red => {
                        self.borrow_mut().left =
                            Some(ATree::build(input, Some(Rc::clone(self)), Color::Black))
                    }
                    Color::Black => {
                        self.borrow_mut().left =
                            Some(ATree::build(input, Some(Rc::clone(self)), Color::Red))
                    }
                }
            } else {
                self.borrow().left.as_ref().unwrap().insert(input);
            }
        } else if num == input {
        } else if self.borrow().right.is_none() {
            let color = self.borrow().color;
            match color {
                Color::Red => {
                    self.borrow_mut().right =
                        Some(ATree::build(input, Some(Rc::clone(self)), Color::Black))
                }
                Color::Black => {
                    self.borrow_mut().right =
                        Some(ATree::build(input, Some(Rc::clone(self)), Color::Red))
                }
            }
        } else {
            self.borrow().right.as_ref().unwrap().insert(input);
        }
    }
}
impl Show for ATree {
    fn show(&self) -> String {
        match (&self.borrow().left, &self.borrow().right) {
            (None, None) => format!("[{}]", self.borrow().num),
            (Some(left), None) => format!("[{},{}]", left.show(), self.borrow().num),
            (None, Some(right)) => format!("[{},{}]", self.borrow().num, right.show()),
            (Some(left), Some(right)) => {
                format!("[{},{},{}]", left.show(), self.borrow().num, right.show())
            }
        }
    }
}
impl Find for ATree {
    fn search(&self, input: i32) -> Option<ATree> {
        let num = self.borrow().num;
        match input.cmp(&num) {
            Ordering::Equal => Some(Rc::clone(self)),
            Ordering::Greater => match &self.borrow().right {
                None => None,
                Some(right) => right.search(input),
            },
            Ordering::Less => match &self.borrow().left {
                None => None,
                Some(left) => left.search(input),
            },
        }
    }
}
fn main() {
    let a = Rc::new(RefCell::new(Tree::default()));
    a.insert(2);
    println!("{}", a.show());
    a.insert(3);
    println!("{}", a.show());
    a.insert(1);
    println!("{}", a.show());
    let b = Rc::new(RefCell::new(Tree::default()));
    b.insert(1);
    b.insert(2);
    b.insert(3);
    println!("{}", b.show());
    let c = a.search(2);
    println!("search");
    if let Some(ref c1) = c {
        println!("{}", c1.show());
    }
    let d = b.search(2);
    println!("search");
    if let Some(ref b1) = d {
        println!("{}", b1.show());
    }
    let e = (&d.as_ref().unwrap().borrow().parent)
        .as_ref()
        .unwrap()
        .show();
    println!("{}", e);
}
