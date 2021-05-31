use std::iter::FromIterator;

#[derive(Default)]
pub struct SimpleLinkedList<T> {
    // Delete this field
    // dummy is needed to avoid unused parameter error during compilation
    // dummy: ::std::marker::PhantomData<T>,
    head: Option<Box<Node<T>>>,
    len: usize,
}

#[derive(Clone)]
pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None, len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, element: T) {
        /*
        let node = Node {
            data: element,
            next: match &self.head {
                None => None,
                Some(head) => Some(head.clone()),
            },
        };
        */
        let node = Node {
            data: element,
            next: self.head.take(),
        };

        self.head = Some(Box::new(node));
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        /*
        if let Some(node) = self.head.clone() {
            self.head = (*node).next;
            self.len -= 1;
            Some((*node).data)
        } else {
            None
        }
        */

        /*
        if let Some(node) = self.head.take() {
            self.head = node.next;
            self.len -= 1;

            Some(node.data)
        } else {
            None
        }
        */

        /*
        let node = self.head.take()?;

        self.head = node.next;
        self.len -= 1;

        Some(node.data)
        */

        self.head.take().map(|node| {
            self.head = node.next;
            self.len -= 1;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        /*
        if let Some(node) = &self.head {
            Some(&(*node).data)
        } else {
            None
        }
        */

        /*
        let node = self.head.as_ref().take()?;
        Some(&(*node).data)
        */

        self.head.as_ref().map(|node| &node.data)
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut s = SimpleLinkedList::new();

        let mut ss = self;

        while let Some(x) = ss.pop() {
            s.push(x)
        }
        s
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut sll = SimpleLinkedList::new();

        for i in iter {
            sll.push(i)
        }

        sll
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut v = Vec::new();

        let mut s = self.rev();

        while let Some(x) = s.pop() {
            v.push(x)
        }

        v
    }
}
