//use std::borrow::Borrow;
//use std::borrow::Borrow;
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
impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        match (&self, other) {
            (&Color::Red, &Color::Red) | (&Color::Black, &Color::Black) => true,
            (_, _) => false,
        }
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
    fn roate(&self);
    fn push(&self, input: i32) -> ATree;
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
    fn push(&self, input: i32) -> ATree {
        self.insert(input);
        let mut temp: ATree = Rc::clone(&self);
        while temp.borrow().parent.is_some() {
            let a = Rc::clone(temp.borrow().parent.as_ref().unwrap());
            temp = a;
        }
        temp.borrow_mut().color = Color::Black;
        temp
    }
    // roate for red and black
    fn roate(&self) {
        let mut z: ATree = Rc::clone(self);
        let num = self.borrow().num;
        //let parent = &self.borrow().parent;
        let has_parent = self.borrow().parent.is_some();
        if has_parent {
            let parent = Rc::clone(self.borrow().parent.as_ref().unwrap());
            // while z.p.color == Red
            let parent_color = parent.borrow().color;
            match parent_color {
                Color::Red => {
                    if parent.borrow().parent.is_some() {
                        let pp = Rc::clone(parent.borrow().parent.as_ref().unwrap());
                        //if let Some(ref pp) = Rc::clone(&parent).borrow().parent {
                        // here to know where the tree is
                        let num_p = parent.borrow().num;
                        // z.p.r
                        let num_p_r = &parent.borrow().right.clone();
                        let num_ppl = &pp.borrow().left.clone();
                        // if z.p == z.p.p.left
                        // and if z.p.p.left is none , it color should be black
                        if num_ppl.is_some() && num_ppl.as_ref().unwrap().borrow().num == num_p {
                            // y = z.p.p.right
                            let y = parent.borrow().right.as_ref().map(Rc::clone);
                            // if y.color == Red
                            // and if y is none , it is also black
                            if y.is_some() && y.as_ref().unwrap().borrow().color == Color::Red {
                                // z.p.color = black
                                //self.borrow_mut().parent.as_mut().unwrap().borrow_mut().color = Color::Black;
                                pp.borrow_mut().color = Color::Red;
                                parent.borrow_mut().color = Color::Black;
                                // y.color= black
                                y.as_ref().unwrap().borrow_mut().color = Color::Black;
                                // z.p.p.color = red

                                z = Rc::clone(&pp);
                            // else if z == z.p.right
                            } else if num_p_r.is_some()
                                && num_p_r.as_ref().unwrap().borrow().num == num
                            {
                                // z = z.p
                                z = Rc::clone(&parent);
                                // it is upper
                                let zp = Rc::clone(z.borrow().parent.as_ref().unwrap());
                                // to get to z
                                let zpp = zp.borrow().parent.as_ref().map(Rc::clone);
                                let zl = z.borrow().left.as_ref().map(Rc::clone);
                                // LEFT-ROTATE(T,z)
                                z.borrow_mut().parent = zpp.clone();
                                z.borrow_mut().left = Some(Rc::clone(&zp));
                                zp.borrow_mut().parent = Some(Rc::clone(&z));
                                zp.borrow_mut().right = zl.clone();


                                //parent.borrow_mut().right = temp;
                            }
                            // RIGHT_ROTATION
                            //if z.borrow().parent.is_none() {
                            //    z.borrow_mut().color = Color::Black;
                            //} else {
                            let zp = Rc::clone(z.borrow().parent.as_ref().unwrap());
                            let zpp = Rc::clone(zp.borrow().parent.as_ref().unwrap());
                            zp.borrow_mut().color = Color::Black;
                            zpp.borrow_mut().color = Color::Red;
                            let ppp = zpp.borrow().parent.as_ref().map(Rc::clone);
                            //let ppr = zpp.borrow().right.as_ref().map(Rc::clone);
                            let pr = zp.borrow().right.as_ref().map(Rc::clone);
                            zp.borrow_mut().parent = ppp.clone();
                            zp.borrow_mut().right = Some(Rc::clone(&zpp));
                            zpp.borrow_mut().parent = Some(Rc::clone(&zp));
                            zpp.borrow_mut().left = pr.clone();
                            z.roate();
                            //}
                        } else {
                            //let pp = parent.borrow().parent.as_ref().map(Rc::clone);
                            //z = Rc::clone(&parent);
                            //println!("ssss");
                            //if self.borrow().left.is_some() {
                            //    let right = Rc::clone(self.borrow().left.as_ref().unwrap());
                            //    parent.borrow_mut().right = Some(right);
                            //} else {
                            //    parent.borrow_mut().right = None;
                            //}
                            //println!("ssss");
                            //parent.borrow_mut().parent = Some(Rc::clone(self));
                            //println!("ssss");
                            //self.borrow_mut().left = Some(Rc::clone(&parent));
                            //self.borrow_mut().parent = pp.clone();
                            //println!("{}",z.borrow().num);
                            //println!("{}",z.borrow().parent.as_ref().unwrap().borrow().left.as_ref().unwrap().borrow().num);
                            //z.roate();
                        }
                    }
                }
                Color::Black => {}
            }
            //}
        }
    }
    fn insert(&self, input: i32) {
        let num = self.borrow().num;
        if num > input {
            if self.borrow().left.is_none() {
                let color = self.borrow().color;
                match color {
                    // here should pay more
                    Color::Red => {
                        self.borrow_mut().left =
                            Some(ATree::build(input, Some(Rc::clone(self)), Color::Red));
                        // roate to make changes
                        let left = Rc::clone(self.borrow().left.as_ref().unwrap());
                        left.roate();
                        //self.borrow().left.as_ref().unwrap().roate();
                    }
                    // if insert is black , it has nonsense
                    Color::Black => {
                        self.borrow_mut().left =
                            Some(ATree::build(input, Some(Rc::clone(self)), Color::Red))
                    }
                }
            } else {
                let left = Rc::clone(self.borrow().left.as_ref().unwrap());
                //self.borrow().left.as_ref().unwrap().insert(input);
                left.insert(input);
            }
        } else if num == input {
        } else if self.borrow().right.is_none() {
            let color = self.borrow().color;
            match color {
                Color::Red => {
                    // according to red black tree, the insert color should all be red.
                    self.borrow_mut().right =
                        Some(ATree::build(input, Some(Rc::clone(self)), Color::Red));
                    let right = Rc::clone(self.borrow().right.as_ref().unwrap());
                    right.roate();
                }
                Color::Black => {
                    self.borrow_mut().right =
                        Some(ATree::build(input, Some(Rc::clone(self)), Color::Red))
                }
            }
        } else {
            let right = Rc::clone(self.borrow().right.as_ref().unwrap());
            right.insert(input);
            //self.borrow().right.as_ref().unwrap().insert(input);
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
    //let b = Rc::new(RefCell::new(Tree::default()));
    //b.insert(1);
    //println!("{}", b.show());
    //b.insert(2);
    //println!("{}", b.show());
    //b.insert(3);
    //println!("{}", b.show());
    //for i in -3..0 {
    //    b.insert(i);
    //    println!("{}", b.show());
    //}
    //println!("{}", b.show());
    //let c = a.search(2);
    //println!("search");
    //if let Some(ref c1) = c {
    //    println!("{}", c1.show());
    //}
    //let d = b.search(2);
    //println!("search");
    //if let Some(ref b1) = d {
    //    println!("{}", b1.show());
    //}
    //let e = (&d.as_ref().unwrap().borrow().parent)
    //    .as_ref()
    //    .unwrap()
    //    .show();
    //println!("{}", e);

    let mut f = Rc::new(RefCell::new(Tree::default()));
    for i in (-8..0).rev() {
        f = f.push(i);
        println!("it is{}", f.show());
    }
    let mut f = Rc::new(RefCell::new(Tree::default()));
    for i in (0..8).rev() {
        f = f.push(i);
        println!("2 it is{}", f.show());
    }
    //println!("{}", f.show());
}
