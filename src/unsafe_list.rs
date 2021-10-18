use std::{fmt::Debug, marker::PhantomData, mem, ptr::NonNull};

/// 节点，用于保存数据
#[derive(Debug)]
pub struct Node<T>
where
    T: Ord,
{
    next: Option<NonNull<Node<T>>>,

    // 数据项
    data: T,
}

impl<T> Node<T>
where
    T: Ord,
{
    fn new(data: T) -> Self {
        Node { next: None, data }
    }
}

impl<T> Node<T>
where
    T: Ord,
{
    fn into_node(self: Box<Self>) -> T {
        self.data
    }
}

/// 链表
pub struct LinkedList<T>
where
    T: Ord,
{
    // 头指针
    head: Option<NonNull<Node<T>>>,

    // 链表长度
    len: usize,
    marker: PhantomData<Box<Node<T>>>,
}

impl<T> LinkedList<T>
where
    T: Ord,
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

    pub fn push_back(&mut self, data: T) {
        // 创建节点，返回指针
        let node = Some(Box::leak(Box::new(Node::new(data))).into());

        // 链表节点数+1
        self.len += 1;

        // 如果链表为空，添加为第一个元素
        if self.len == 0 {
            self.head = node;
            return;
        }

        // 临时指针，用于每次循环
        let mut p = self.head;

        loop {
            let mut flag = false;

            let mut cur = p.unwrap().as_ptr();
            let mut t = unsafe { Box::from_raw(cur) };

            let v = t.next;
            if v == None {
                t.next = node;
                flag = true;
            }
            p = v;
            cur = Box::leak(t.into());
            if flag {
                break;
            }
        }
    }

    /// 从头部获取一个元素
    pub fn pop(&mut self) -> Option<T> {
        let v = if let Some(v) = self.head {
            unsafe { Box::from_raw(v.as_ptr()) }
        } else {
            return None;
        };
        self.head = v.next;
        self.len -= 1;
        Some(v.into_node())
    }

    /// 从尾部获取一个元素
    pub fn pop_tail(&mut self) -> Option<T> {
        // 没节点的情况
        if self.len == 0 {
            return None;
        } else if self.len == 1 {
        }
        None
    }

    /// 逆序链表
    pub fn rev(&mut self) {}

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
    T: Ord,
{
    fn drop(&mut self) {
        struct DropGuard<'a, T>(&'a mut LinkedList<T>)
        where
            T: Ord;

        impl<'a, T> Drop for DropGuard<'a, T>
        where
            T: Ord,
        {
            // 当下面的循环出现错误的时候，这个drop就会被执行，继续释放所有的元素，
            // 下面的while循环正常执行的时候，由于是调用forget方法，他不会调用drop函数，这里的drop就不会被调用。
            fn drop(&mut self) {
                while self.0.pop().is_some() {}
            }
        }

        while let Some(node) = self.pop() {
            // println!("drop: {:?}", &node as *const T);
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
    T: Ord,
{
    // 记录当前遍历的节点
    cur: Option<NonNull<Node<T>>>,
    marker: PhantomData<Node<&'a T>>,
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: Ord,
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
    fn list_iter() {
        let mut list = LinkedList::new();

        list.push("a".to_string());
        list.push_back("b".to_string());
        list.push("c".to_string());
        list.push_back("d".to_string());

        let res: Vec<&String> = list.iter().collect();

        assert_eq!(vec!["c", "a", "b", "d"], res);
    }

    // #[test]
    // fn list_pop() {
    //     let mut list = LinkedList::new();
    //     list.push(1);
    //     list.push(2);
    //     list.push(3);
    //     list.push(4);

    //     while let Some(v) = list.pop_tail() {
    //         println!("{:?}", v);
    //     }
    // }
}
