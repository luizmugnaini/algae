use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

// Each node should have exactly two pointers to it. Each node in the middle of
// the list is pointed at by its predecessor and successor, while the nodes on
// the ends are pointed to by the list itself.
//
// `head`: has an empty prev and some next;
// `tail`: has some next and an empty prev;
pub struct DoubleLinked<T> {
    head: List<T>,
    tail: List<T>,
}

impl<T> DoubleLinked<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, key: T) {
        let new_head = DoubleNode::new(key);
        match self.head.take() {
            Some(last_head) => {
                // Cloning an Rc just creates a new pointer to the same object
                last_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(last_head);
                self.head = Some(new_head);
            }
            None => {
                self.head = Some(new_head.clone());
                self.tail = Some(new_head);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        // Pop only when the list is nonempty
        self.head.take().map(|last_head| {
            // Check for the node head points to (next), the next node will
            // become the new head of the list
            match last_head.borrow_mut().next.take() {
                Some(new_head) => {
                    // Empty the previous node of the new head (which is the
                    // last head) and update the head
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    // head was the only node, remove the last pointer to it
                    self.tail.take();
                }
            }
            Rc::try_unwrap(last_head).ok().unwrap().into_inner().key
        })
    }

    pub fn push_back(&mut self, key: T) {
        let new_tail = DoubleNode::new(key);
        match self.tail.take() {
            Some(last_tail) => {
                last_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(last_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.tail = Some(new_tail.clone());
                self.head = Some(new_tail);
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|last_tail| {
            match last_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next = None;
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(last_tail).ok().unwrap().into_inner().key
        })
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    // This is not a good peek because we return an option to a Ref<T>, which
    // holds the key, but there is no turn around for that
    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.key))
    }
}

impl<T> Default for DoubleLinked<T> {
    fn default() -> Self {
        Self::new()
    }
}

type List<T> = Option<Rc<RefCell<DoubleNode<T>>>>;

struct DoubleNode<T> {
    key: T,
    next: List<T>,
    prev: List<T>,
}

impl<T> DoubleNode<T> {
    fn new(key: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            key,
            next: None,
            prev: None,
        }))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn front_push_n_pop() {
        let mut list = DoubleLinked::new();
        assert_eq!(list.pop_front(), None);
        for x in 0..3 {
            list.push_front(x);
        }
        for x in (0..3).rev() {
            assert_eq!(list.pop_front(), Some(x));
        }
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn back_push_n_pop() {
        let mut list = DoubleLinked::new();
        assert_eq!(list.pop_back(), None);
        for x in 0..3 {
            list.push_back(x);
        }
        for x in (0..3).rev() {
            assert_eq!(list.pop_back(), Some(x));
        }
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn is_empty() {
        let mut list = DoubleLinked::new();
        assert!(list.is_empty());

        list.push_front(3);
        assert!(!list.is_empty());

        // ensure that `is_empty` call didn't mess up with the list
        assert!(!list.is_empty());

        list.pop_front();
        assert!(list.is_empty());
    }

    #[test]
    fn peek_front() {
        let mut list = DoubleLinked::new();

        // We cant test the Option<Ref<'_, T>> for equality directly
        let check_empty = |node: Option<Ref<'_, i32>>| {
            if node.is_some() {
                panic!("Node should be None.");
            }
        };
        check_empty(list.peek_front());

        for x in 0..3 {
            list.push_front(x);
        }
        for x in (0..3).rev() {
            assert_eq!(&*list.peek_front().unwrap(), &x);
            list.pop_front();
        }

        check_empty(list.peek_front());
    }
}
