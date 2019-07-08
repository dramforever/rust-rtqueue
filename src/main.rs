use std::iter::FromIterator;
use std::cell::{Cell, RefCell};
use std::convert::TryInto;
use std::fmt;
use std::io;
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
struct Queue<T> {
    front: Option<Rc<Node<T>>>,
    back: Option<Rc<Node<T>>>,
    jump: Option<Rc<Node<T>>>,
}

impl<T: Clone> Queue<T> {
    fn new() -> Queue<T> {
        Queue {
            front: None,
            back: None,
            jump: None,
        }
    }

    fn pop_front(&self) -> Option<(Queue<T>, T)> {
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

    fn push_back(&self, v: T) -> Queue<T> {
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

struct QueueIter<T> {
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

#[test]
fn test_queue() {
    let a = Queue::new()
        .push_back(1)
        .push_back(2);
    let b = a
        .push_back(3)
        .push_back(4);
    let c = a
        .push_back(5)
        .push_back(6);
    let b1 = b
        .pop_front().unwrap().0
        .pop_front().unwrap().0;
    let c1 = c
        .pop_front().unwrap().0
        .pop_front().unwrap().0;

    assert!(Vec::from_iter(&a) == vec![ 1, 2 ]);
    assert!(Vec::from_iter(&b) == vec![ 1, 2, 3, 4 ]);
    assert!(Vec::from_iter(&c) == vec![ 1, 2, 5, 6 ]);
    assert!(Vec::from_iter(&b1) == vec![ 3, 4 ]);
    assert!(Vec::from_iter(&c1) == vec![ 5, 6 ]);
}

fn main() {
    fn parse_line() -> Vec<u32> {
        let mut line: String = "".to_string();
        io::stdin().read_line(&mut line)
            .expect("read_line");

        line.split_whitespace()
            .map(|val| { val.parse().expect("Parse") })
            .collect()
    }

    let vheader = parse_line();
    assert!(vheader.len() == 2);

    let n = vheader[0];
    let ty = vheader[1];

    let mut hash: u32 = 0;


    let ver_size: usize = (n + 1).try_into()
        .expect("main: size too large");

    let mut version: Vec<Queue<u32>> =
        Vec::with_capacity(ver_size);

    version.push(Queue::new());

    for _ in 1 ..= n {
        let vaction = parse_line();

        let h = if ty == 1 { hash } else { 0 };

        if vaction[0] == 1 {
            assert!(vaction.len() == 3);
            let v = vaction[1] ^ h;
            let t = vaction[2] ^ h;
            version.push(version[v as usize].push_back(t));
        } else {
            assert!(vaction[0] == 2);
            assert!(vaction.len() == 2);

            let v = vaction[1] ^ h;

            let (res_queue, res_val) = version[v as usize].pop_front()
                .expect("main: queue is empty");

            version.push(res_queue);

            hash = hash.wrapping_mul(31).wrapping_add(res_val);
        }
    }

    println!("{}", hash);

    for (i, q) in version.iter().enumerate() {
        println!("{} {:?}", i, q);
    }
}
