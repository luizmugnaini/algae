struct Node<T> {
    key: T,
    next: Option<Box<Node<T>>>,
}

type List<T> = Option<Box<Node<T>>>;

impl<T> Node<T> {
    fn new(key: T, next: List<T>) -> Self {
        Node { key, next }
    }
}

pub struct SingleLinkedList<T> {
    head: List<T>,
}

impl<T> SingleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, key: T) {
        let node = Box::new(Node::new(key, self.head.take()));
        self.head = Some(node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.key
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.key)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.key)
    }

    pub fn iter(&self) -> SingleLinkedListIter<T> {
        SingleLinkedListIter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> SingleLinkedListIterMut<T> {
        SingleLinkedListIterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<T> Default for SingleLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for SingleLinkedList<T> {
    fn drop(&mut self) {
        let mut list = self.head.take();
        while let Some(mut node) = list {
            list = node.next.take();
        }
    }
}

pub struct SingleLinkedListIntoIter<T>(SingleLinkedList<T>);

impl<T> Iterator for SingleLinkedListIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<T> IntoIterator for SingleLinkedList<T> {
    type Item = T;
    type IntoIter = SingleLinkedListIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        SingleLinkedListIntoIter(self)
    }
}

pub struct SingleLinkedListIter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for SingleLinkedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.key
        })
    }
}

pub struct SingleLinkedListIterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for SingleLinkedListIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.key
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn push_n_pop() {
        let mut list = SingleLinkedList::new();

        // Verify the correctness of pop for the empty case
        assert_eq!(list.pop(), None);

        for x in 0..4 {
            list.push(x);
        }

        for x in (0..4).rev() {
            assert_eq!(list.pop(), Some(x));
        }

        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = SingleLinkedList::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);

        list.push(1);
        assert_eq!(list.peek(), Some(&1));

        list.peek_mut().map(|key| {
            *key += 2;
        });
        assert_eq!(list.peek_mut(), Some(&mut 3));
        assert_eq!(list.peek(), Some(&3));
    }

    #[test]
    fn into_iter() {
        let mut list = SingleLinkedList::new();
        for x in 0..3 {
            list.push(x);
        }
        let mut iter = list.into_iter();
        for x in (0..3).rev() {
            assert_eq!(iter.next(), Some(x));
        }
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = SingleLinkedList::new();
        for x in 0..3 {
            list.push(x);
        }
        let mut iter = list.iter();
        for x in (0..3).rev() {
            assert_eq!(iter.next(), Some(&x));
        }
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = SingleLinkedList::new();
        for x in 0..3 {
            list.push(x);
        }
        let mut iter = list.iter_mut();
        for mut x in (0..3).rev() {
            assert_eq!(iter.next(), Some(&mut x));
        }
        assert_eq!(iter.next(), None);
    }
}
