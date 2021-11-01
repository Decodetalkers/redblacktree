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
impl Color {
    fn show_color(&self) -> String {
        match *self {
            Color::Red => "Red".to_string(),
            Color::Black => "Black".to_string(),
        }
    }
}
#[derive(Default)]
pub struct Tree {
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
pub type ATree = Rc<RefCell<Tree>>;
pub trait Build {
    fn build() -> Self;
}
trait Insert {
    fn abuild(num: i32, parent: Option<Rc<RefCell<Tree>>>, color: Color) -> Self;
    fn insert(&self, input: i32);
    fn roate(&self);
    //fn push(&self, input: i32) -> ATree;
}
pub trait Push {
    fn push(&self, input: i32) -> Self;
}
pub trait Find {
    fn search(&self, input: i32) -> Option<ATree>;
}
pub trait Show {
    fn show(&self) -> String;
    fn show_color(&self) -> String;
}
impl Push for ATree {
    fn push(&self, input: i32) -> Self {
        self.insert(input);
        let mut temp: ATree = Rc::clone(self);
        while temp.borrow().parent.is_some() {
            let a = Rc::clone(temp.borrow().parent.as_ref().unwrap());
            temp = a;
        }
        temp.borrow_mut().color = Color::Black;
        temp
    }
}
impl Build for ATree {
    fn build() -> Self {
        Rc::new(RefCell::new(Tree::default()))
    }
}
impl Insert for ATree {
    #[inline]
    fn abuild(num: i32, parent: Option<Rc<RefCell<Tree>>>, color: Color) -> Self {
        Rc::new(RefCell::new(Tree::new(num, parent, color)))
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
                        let num_p_l = &parent.borrow().left.clone();
                        let num_ppl = &pp.borrow().left.clone();
                        // if z.p == z.p.p.left
                        // and if z.p.p.left is none , it color should be black
                        if num_ppl.is_some() && num_ppl.as_ref().unwrap().borrow().num == num_p {
                            // y = z.p.p.right
                            let y = pp.borrow().right.as_ref().map(Rc::clone);
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
                                let beta = self.borrow().right.as_ref().map(Rc::clone);
                                self.borrow_mut().parent = Some(Rc::clone(&pp));
                                if pp.borrow().left.is_some()
                                    && pp.borrow().left.as_ref().unwrap().borrow().num == num_p
                                {
                                    pp.borrow_mut().left = Some(Rc::clone(self));
                                } else {
                                    pp.borrow_mut().right = Some(Rc::clone(self));
                                }
                                self.borrow_mut().left = Some(Rc::clone(&z));
                                z.borrow_mut().parent = Some(Rc::clone(self));
                                z.borrow_mut().right = beta.clone();
                                if beta.is_some() {
                                    beta.as_ref().unwrap().borrow_mut().parent =
                                        Some(Rc::clone(&z));
                                }
                            }
                            // RIGHT_ROTATION
                            if !(z.borrow().parent.is_none()
                                || z.borrow()
                                    .parent
                                    .as_ref()
                                    .unwrap()
                                    .as_ref()
                                    .borrow()
                                    .parent
                                    .is_none())
                            {
                                //println!("debug: {}",z.as_ref().borrow().num);
                                //println!("debug: {}",z.as_ref().borrow().parent.as_ref().unwrap().borrow().num);
                                let zp = Rc::clone(z.borrow().parent.as_ref().unwrap());
                                let zpp = Rc::clone(zp.borrow().parent.as_ref().unwrap());
                                zp.borrow_mut().color = Color::Black;
                                zpp.borrow_mut().color = Color::Red;
                                let ppp = zpp.borrow().parent.as_ref().map(Rc::clone);
                                let beta = zp.borrow().right.as_ref().map(Rc::clone);
                                zp.borrow_mut().parent = ppp.clone();
                                if ppp.is_some() {
                                    let ppparent = Rc::clone(ppp.as_ref().unwrap());
                                    let isleft = ppparent.borrow().left.is_some()
                                        && ppparent.borrow().left.as_ref().unwrap().borrow().num
                                            == zpp.borrow().num;
                                    if isleft {
                                        ppparent.borrow_mut().left = Some(Rc::clone(&zp));
                                    } else {
                                        ppparent.borrow_mut().right = Some(Rc::clone(&zp));
                                    }
                                }
                                zp.borrow_mut().right = Some(Rc::clone(&zpp));
                                zpp.borrow_mut().parent = Some(Rc::clone(&zp));
                                zpp.borrow_mut().left = beta.clone();
                                if beta.is_some() {
                                    beta.as_ref().unwrap().borrow_mut().parent =
                                        Some(Rc::clone(&zpp));
                                }

                                z.roate();
                            }
                        } else {
                            // same as is left
                            let y = pp.borrow().left.as_ref().map(Rc::clone);
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
                            } else if num_p_l.is_some()
                                && num_p_l.as_ref().unwrap().borrow().num == num
                            {
                                // z = z.p
                                z = Rc::clone(&parent);
                                // it is upper
                                let beta = self.borrow().left.as_ref().map(Rc::clone);
                                self.borrow_mut().parent = Some(Rc::clone(&pp));
                                if pp.borrow().right.is_some()
                                    && pp.borrow().right.as_ref().unwrap().borrow().num == num_p
                                {
                                    pp.borrow_mut().right = Some(Rc::clone(self));
                                } else {
                                    pp.borrow_mut().left = Some(Rc::clone(self));
                                }
                                self.borrow_mut().right = Some(Rc::clone(&z));
                                z.borrow_mut().parent = Some(Rc::clone(self));
                                z.borrow_mut().left = beta.clone();
                                if beta.is_some() {
                                    beta.as_ref().unwrap().borrow_mut().parent =
                                        Some(Rc::clone(&z));
                                }
                            }
                            // RIGHT_ROTATION
                            if !(z.borrow().parent.is_none()
                                || z.borrow()
                                    .parent
                                    .as_ref()
                                    .unwrap()
                                    .as_ref()
                                    .borrow()
                                    .parent
                                    .is_none())
                            {
                                //println!("debug: {}",z.as_ref().borrow().num);
                                //println!("debug: {}",z.as_ref().borrow().parent.as_ref().unwrap().borrow().num);
                                let zp = Rc::clone(z.borrow().parent.as_ref().unwrap());
                                let zpp = Rc::clone(zp.borrow().parent.as_ref().unwrap());
                                zp.borrow_mut().color = Color::Black;
                                zpp.borrow_mut().color = Color::Red;
                                let ppp = zpp.borrow().parent.as_ref().map(Rc::clone);
                                let beta = zp.borrow().left.as_ref().map(Rc::clone);
                                zp.borrow_mut().parent = ppp.clone();
                                if ppp.is_some() {
                                    let ppparent = Rc::clone(ppp.as_ref().unwrap());
                                    let isright = ppparent.borrow().right.is_some()
                                        && ppparent.borrow().right.as_ref().unwrap().borrow().num
                                            == zpp.borrow().num;
                                    if isright {
                                        ppparent.borrow_mut().right = Some(Rc::clone(&zp));
                                    } else {
                                        ppparent.borrow_mut().left = Some(Rc::clone(&zp));
                                    }
                                }
                                zp.borrow_mut().left = Some(Rc::clone(&zpp));
                                zpp.borrow_mut().parent = Some(Rc::clone(&zp));
                                zpp.borrow_mut().right = beta.clone();
                                if beta.is_some() {
                                    beta.as_ref().unwrap().borrow_mut().parent =
                                        Some(Rc::clone(&zpp));
                                }

                                z.roate();
                            }
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
                            Some(ATree::abuild(input, Some(Rc::clone(self)), Color::Red));
                        // roate to make changes
                        let left = Rc::clone(self.borrow().left.as_ref().unwrap());
                        left.roate();
                        //self.borrow().left.as_ref().unwrap().roate();
                    }
                    // if insert is black , it has nonsense
                    Color::Black => {
                        self.borrow_mut().left =
                            Some(ATree::abuild(input, Some(Rc::clone(self)), Color::Red))
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
                        Some(ATree::abuild(input, Some(Rc::clone(self)), Color::Red));
                    let right = Rc::clone(self.borrow().right.as_ref().unwrap());
                    right.roate();
                }
                Color::Black => {
                    self.borrow_mut().right =
                        Some(ATree::abuild(input, Some(Rc::clone(self)), Color::Red))
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
    fn show_color(&self) -> String {
        match (&self.borrow().left, &self.borrow().right) {
            (None, None) => format!("[{}]", self.borrow().color.show_color()),
            (Some(left), None) => format!(
                "[{},{}]",
                left.show_color(),
                self.borrow().color.show_color()
            ),
            (None, Some(right)) => format!(
                "[{},{}]",
                self.borrow().color.show_color(),
                right.show_color()
            ),
            (Some(left), Some(right)) => {
                format!(
                    "[{},{},{}]",
                    left.show_color(),
                    self.borrow().color.show_color(),
                    right.show_color()
                )
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
