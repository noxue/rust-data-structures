use std::ptr::NonNull;

struct Node<T> {
    next: Option<NonNull<Node<T>>>,
    element: T,
}

impl<T> Node<T> {
    fn new(element: T) -> Self {
        Node {
            next: None,
            element,
        }
    }
}

struct List<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    // marker: PhantomData<Box<Node<T>>>,
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        struct DropGuard<'a, T>(&'a mut List<T>);

        impl<'a, T> Drop for DropGuard<'a, T> {
            fn drop(&mut self) {
                // Continue the same loop we do below. This only runs when a destructor has
                // panicked. If another one panics this will abort.
                println!("pppppppppppppp");
                while self.0.pop_front().is_some() {}
            }
        }

        while let Some(v) = self.pop_front() {
            let guard = DropGuard(self);
            drop(v);
            // panic!("xx");
            std::mem::forget(guard);
        }
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
            len: 0,
            // marker: PhantomData,
        }
    }

    #[inline]
    fn push_front_node(&mut self, mut node: Box<Node<T>>) {
        node.next = self.head;

        let node = Some(Box::leak(node).into());
        if self.head == None {
            self.tail = node;
        }
        self.head = node;
        self.len += 1;
    }

    pub fn push_front(&mut self, ele: T) {
        self.push_front_node(Box::new(Node::new(ele)));
    }

    pub fn push_back(&mut self, ele: T) {
        self.push_back_node(Box::new(Node::new(ele)));
    }

    #[inline]
    fn push_back_node(&mut self, node: Box<Node<T>>) {
        let node = Some(Box::leak(node).into());
        if let Some(v) = self.tail {
            unsafe { (*v.as_ptr()).next = node };
        }
        if self.head == None {
            self.head = node;
        }
        self.tail = node;
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<Box<Node<T>>> {
        self.head.map(|v| unsafe {
            let node = Box::from_raw(v.as_ptr());
            self.head = node.next;
            node
        })
    }
}

// fn main() {
//     let mut i = 0;
//     loop {
//         // let mut list = LinkedList::new();
//         // for i in 0..=1024 * 1024 {
//         //     list.push_front(i);
//         // }

//         let mut list = List::new();

//         for i in 0..=1024 * 1024 * 10 {
//             list.push_back(i);
//         }

//         // while let Some(v) = list.pop_front() {
//         //     println!("{:#?}", v.element);
//         // }

//         println!("{}", i);
//         i += 1;
//     }
// }

#[cfg(test)]
mod tests {
    use crate::List;

    #[test]
    fn test_list() {
        let mut list = List::new();

        for i in 0..=5 {
            list.push_back(i);
        }

        for i in 0..=5 {
            assert_eq!(list.pop_front().unwrap().element, i);
        }
        
    }
}
