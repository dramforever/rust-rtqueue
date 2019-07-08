use std::cell::{Cell, RefCell};
use std::fmt;
use std::rc::Rc;

#[derive(Clone)]
enum Zipper<T> {
    Clean,
    Dirty(Rc<Node<T>>, Option<Rc<Node<T>>>),
}

struct Node<T> {
    value: T,
    next: RefCell<Option<Rc<Node<T>>>>,
    zipper: Cell<Zipper<T>>,
}

impl<T: Clone> Node<T> {
    fn create_lazy(x: &Option<Rc<Node<T>>>, y: Option<Rc<Node<T>>>) -> Option<Rc<Node<T>>> {
        if let Some(x) = x {
            let y = y.expect("create_lazy: imbalance");

            Some(Rc::new(Node {
                value: x.value.clone(),
                next: x.next.clone(),
                zipper: Cell::new(Zipper::Dirty(y, None)),
            }))
        } else {
            y
        }
    }

    fn rotate_zipper(&self) {
        if let Zipper::Dirty(c, d) = self.zipper.replace(Zipper::Clean) {

            let node = Rc::new(Node {
                value: c.value.clone(),
                next: RefCell::new(d),
                zipper: Cell::new(Zipper::Clean),
            });

            let c_next = c.next.borrow().clone()
                .expect("rotate_zipper: imbalance");

            let new_next =
                if let Some(next) = self.next.borrow().clone() {
                    Node {
                        value: next.value.clone(),
                        next: next.next.clone(),
                        zipper: Cell::new(
                            Zipper::Dirty(c_next, Some(node))),
                    }
                } else {
                    Node {
                        value: c_next.value.clone(),
                        next: RefCell::new(Some(node)),
                        zipper: Cell::new(Zipper::Clean)
                    }
                };

            self.next.replace(Some(Rc::new(new_next)));
        }
    }
}

#[derive(Clone)]
pub struct Queue<T> {
    front: Option<Rc<Node<T>>>,
    back: Option<Rc<Node<T>>>,
    jump: Option<Rc<Node<T>>>,
}

impl<T: Clone> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            front: None,
            back: None,
            jump: None,
        }
    }
}

impl<T: Clone> Queue<T> {
    pub fn new() -> Queue<T> {
        Default::default()
    }

    pub fn pop_front(&self) -> Option<(Queue<T>, T)> {
        let front = self.front.as_ref()?;
        let res = front.value.clone();

        let res_queue =
            if let Some(jump) = &self.jump {
                jump.rotate_zipper();

                Queue {
                    front: front.next.borrow().clone(),
                    back: self.back.clone(),
                    jump: jump.next.borrow().clone(),
                }
            } else {
                let zipper = Node::create_lazy(&front.next.borrow(), self.back.clone());

                Queue {
                    front: zipper.clone(),
                    back: None,
                    jump: zipper,
                }
            };

        Some((res_queue, res))
    }

    pub fn push_back(&self, v: T) -> Queue<T> {
        let new_node = Rc::new(Node {
            value: v,
            next: RefCell::new(self.back.clone()),
            zipper: Cell::new(Zipper::Clean),
        });

        if let Some(jump) = &self.jump {
            jump.rotate_zipper();

            Queue {
                front: self.front.clone(),
                back: Some(new_node),
                jump: jump.next.borrow().clone(),
            }
        } else {
            let zipper = Node::create_lazy(&self.front, Some(new_node));

            Queue {
                front: zipper.clone(),
                back: None,
                jump: zipper,
            }
        }
    }
}

pub struct QueueIter<T> {
    dat: Queue<T>
}

impl<T: Clone> Iterator for QueueIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let (next, res) = self.dat.pop_front()?;
        self.dat = next;
        Some(res)
    }
}

impl<T: Clone> IntoIterator for &'_ Queue<T> {
    type Item = T;
    type IntoIter = QueueIter<T>;

    fn into_iter(self) -> QueueIter<T> {
        QueueIter { dat: self.clone() }
    }
}

impl<T: fmt::Debug + Clone> fmt::Debug for Queue<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}
