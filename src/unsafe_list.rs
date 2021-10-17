use std::{
    cell::RefCell,
    fmt::Debug,
    marker::{self, PhantomData},
    mem,
    ptr::NonNull,
    rc::Rc,
};

/// 节点，用于保存数据
#[derive(Debug)]
pub struct Node<T>
where
    T: Debug,
{
    next: Option<NonNull<Node<T>>>,

    // 数据项
    data: T,
}

impl<T> Node<T>
where
    T: Debug,
{
    fn new(data: T) -> Self {
        Node { next: None, data }
    }
}

impl<T> Drop for Node<T>
where
    T: Debug,
{
    fn drop(&mut self) {
        println!("drop:{:?}", self.data)
    }
}

/// 链表
pub struct LinkedList<T>
where
    T: Debug,
{
    // 头指针
    head: Option<NonNull<Node<T>>>,

    // 链表长度
    len: usize,
    marker: PhantomData<Box<Node<T>>>,
}

impl<T> LinkedList<T>
where
    T: Debug,
{
    /// 新建链表
    pub fn new() -> Self {
        LinkedList {
            head: None,
            len: 0,
            marker: PhantomData,
        }
    }

    /// 头部插入数据
    pub fn push(&mut self, data: T) {
        // 先创建好数据项
        let mut data = Box::new(Node::new(data));

        if let Some(head) = self.head {
            data.next = Some(head);
        }
        self.head = Some(Box::leak(data).into());
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<Node<T>> {
        unsafe {
            let v = if let Some(v) = self.head {
                *Box::from_raw(v.as_ptr())
            } else {
                return None;
            };
            self.head = v.next;
            Some(v)
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        let v = *self.head.as_ref().unwrap();
        Iter {
            cur: Some(v),
            marker: PhantomData,
        }
    }
}

impl<T> Drop for LinkedList<T>
where
    T: Debug,
{
    fn drop(&mut self) {
        struct DropGuard<'a, T>(&'a mut LinkedList<T>)
        where
            T: Debug;

        impl<'a, T> Drop for DropGuard<'a, T>
        where
            T: Debug,
        {
            // 当下面的循环出现错误的时候，这个drop就会被执行，继续释放所有的元素， 
            // 下面的while循环正常执行的时候，由于是调用forget方法，他不会调用drop函数，这里的drop就不会被调用。
            fn drop(&mut self) {
                while self.0.pop().is_some() {}
            }
        }

        while let Some(node) = self.pop() {
            let guard = DropGuard(self);
            drop(node);
            mem::forget(guard);
        }
    }
}

// impl<I: Iterator> IntoIterator for I
// where
//     I: Debug,
// {
//     type Item = I::Item;

//     type IntoIter = I;

//     fn into_iter(self) -> I {
//         self.iter()
//     }
// }

pub struct Iter<'a, T: 'a>
where
    T: Debug,
{
    // 记录当前遍历的节点
    cur: Option<NonNull<Node<T>>>,
    marker: PhantomData<Node<&'a T>>,
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: Debug,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if let None = self.cur {
            return None;
        }
        unsafe {
            let v = &(&*self.cur.unwrap().as_ref()).data;
            self.cur = (*self.cur.unwrap().as_ptr()).next;
            Some(v)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::LinkedList;

    #[test]
    fn list_new() {
        let mut list = LinkedList::new();
        list.push("a".to_string());
        list.push("b".to_string());
        list.push("c".to_string());

        let mut node = list.head;
        while let Some(v) = node {
            unsafe {
                let d = &*v.as_ptr();
                println!("{}", d.data);
                node = d.next;
            }
        }

        println!("1 finish");

        let mut node = list.head;
        while let Some(v) = node {
            unsafe {
                let d = &*v.as_ptr();
                println!("{}", d.data);
                node = d.next;
            }
        }

        println!("2 finish");
    }

    #[test]
    fn list_iter() {
        let mut list = LinkedList::new();
        list.push("a".to_string());
        list.push("b".to_string());
        list.push("c".to_string());

        for v in list.iter() {
            println!("{}", v);
        }
    }

    #[test]
    fn list_pop() {
        let mut list = LinkedList::new();
        list.push("a".to_string());
        list.push("b".to_string());
        list.push("c".to_string());

        while let Some(v) = list.pop() {
            println!("{:?}", v.data);
        }
    }
}
