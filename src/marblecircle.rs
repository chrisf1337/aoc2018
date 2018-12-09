use std::{cell::RefCell, fmt, rc::Rc};

type NodeTy<T> = Rc<RefCell<Node<T>>>;

#[derive(Clone, Debug)]
struct Node<T>
where
    T: fmt::Debug + PartialEq + Eq,
{
    pub prev: Option<NodeTy<T>>,
    pub next: Option<NodeTy<T>>,
    pub val: T,
}

impl<T: fmt::Debug + PartialEq + Eq> PartialEq for Node<T> {
    fn eq(&self, other: &Node<T>) -> bool {
        self.val == other.val
    }
}

impl<T: fmt::Debug + PartialEq + Eq> Eq for Node<T> {}

impl<T: fmt::Debug + PartialEq + Eq> Node<T> {
    fn new(val: T) -> NodeTy<T> {
        Rc::new(RefCell::new(Node {
            prev: None,
            next: None,
            val,
        }))
    }

    fn new_with_prev_next(val: T, prev: Option<NodeTy<T>>, next: Option<NodeTy<T>>) -> NodeTy<T> {
        Rc::new(RefCell::new(Node { prev, next, val }))
    }
}

#[derive(Clone)]
pub struct MarbleCircle<T>
where
    T: fmt::Debug + PartialEq + Eq,
{
    node: NodeTy<T>,
    len: usize,
}

impl<T: fmt::Debug + PartialEq + Eq + Clone> fmt::Debug for MarbleCircle<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let start = self.node.clone();
        let mut vals = {
            let start = start.borrow();
            vec![start.val.clone()]
        };
        if self.len >= 2 {
            let node = start.borrow();
            let mut node = node.next.clone().unwrap();
            while node != start {
                node = {
                    let node_borrow = node.borrow();
                    vals.push(node_borrow.val.clone());
                    node_borrow.next.clone().unwrap()
                };
            }
        }
        fmt.debug_list().entries(vals).finish()
    }
}

impl<T: fmt::Debug + PartialEq + Eq + Clone> MarbleCircle<T> {
    pub fn new(val: T) -> MarbleCircle<T> {
        let node = Node::new(val);
        MarbleCircle { node, len: 1 }
    }

    pub fn move_right(&mut self) {
        if self.len == 1 {
            return;
        }
        self.node = {
            let node = self.node.borrow();
            node.next.clone().unwrap()
        };
    }

    pub fn move_left(&mut self) {
        if self.len == 1 {
            return;
        }
        self.node = {
            let node = self.node.borrow();
            node.prev.clone().unwrap()
        };
    }

    pub fn insert(&mut self, val: T) {
        let next = self.node.clone();
        let prev = if self.len == 1 {
            self.node.clone()
        } else {
            let node = self.node.borrow();
            node.prev.clone().unwrap()
        };
        let new_node = Node::new_with_prev_next(val, Some(prev.clone()), Some(next.clone()));
        prev.borrow_mut().next = Some(new_node.clone());
        next.borrow_mut().prev = Some(new_node.clone());
        self.node = new_node;
        self.len += 1;
    }

    pub fn remove(&mut self) -> T {
        let next = {
            let node = self.node.borrow();
            node.next.clone().unwrap()
        };
        let prev = {
            let node = self.node.borrow();
            node.prev.clone().unwrap()
        };
        prev.borrow_mut().next = Some(next.clone());
        next.borrow_mut().prev = Some(prev.clone());
        let val = {
            let node = self.node.borrow();
            node.val.clone()
        };
        self.node = next;
        self.len -= 1;
        val
    }
}
