

use std::{borrow::Borrow, cell::RefCell, marker::PhantomData, ops::Deref, rc::{Rc, Weak}};

/// 节点，用于保存数据
struct Node<T> {
    // Option 用于表示是否有下一个节点
    // Rc  是因为同一个数据可能被多个地方引用
    // RefCell 是用于在一个可变的变量中保存可变数据，这里用于不改变next但是改变next中的node
    next: Option<Rc<RefCell<Node<T>>>>,

    // 数据项
    data: T,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node { next: None, data }
    }
}

/// 链表
pub struct LinkedList<T> {
    // 头指针
    head: Option<Rc<RefCell<Node<T>>>>,

    // 链表长度
    len: usize,
}

impl<T> LinkedList<T> {
    /// 新建链表
    pub fn new() -> Self {
        LinkedList { head: None, len: 0 }
    }

    /// 头部插入数据
    pub fn push(&mut self, data: T) {
        // 先创建好数据项
        let data = Some(Rc::new(RefCell::new(Node::new(data))));

        // 如果是空链表，直接把他当做第一个元素
        if self.len == 0 {
            self.head = data;
            self.len += 1;
            return;
        }

        // 如果链表不为空, 把当前节点的next指向链表第一个节点，然后把当前节点赋值给链表第一个节点
        // 这里要注意使用take，拿到head指向的数据节点的所有权，如果不适用take，后面无法给self.head赋值
        RefCell::borrow_mut(&data.clone().unwrap()).next = self.head.take();
        self.head = data;
        self.len += 1;
    }

 
    pub fn iter(&self)->Iter<'_, T>{
        let v = self.head.as_ref().unwrap().borrow_mut().deref();
        Iter{ cur: v, marker:PhantomData}
    }

}

// impl<'a, T> IntoIterator for &'a LinkedList<T>{
//     type Item = &'a T;

//     type IntoIter = Iter<'a, T>;

//     fn into_iter(self) -> Self::IntoIter {
        
//     }
// }

pub struct Iter<'a, T: 'a> {
    // 记录当前遍历的节点
    cur: Node<&'a T>,
    marker:PhantomData<Node<&'a T>>
}

// impl<'a, T> Iterator for Iter<'a, T> {
//     type Item = &'a T;

//     fn next(&mut self) -> Option<&'a T> {
//         if let None = self.cur {
//             return None;
//         }

//         // let v = Some(&);
//         // self.cur = Some(self.cur.unwrap());
        
//         Some(&RefCell::borrow(self.cur.as_ref().unwrap().as_ptr()).data)
//     }
// }


#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use super::LinkedList;

    #[test]
    fn list_new() {
        let mut list = LinkedList::new();
        list.push("a".to_string());
        list.push("b".to_string());
        list.push("c".to_string());

        let mut node = list.head;
        while let Some(v) = node {
            println!("{}", v.clone().borrow_mut().data);
            node = v.clone().borrow_mut().next.clone();
        }
    }

    #[test]
    fn list_iter(){
        let mut list = LinkedList::new();
        list.push("a".to_string());
        list.push("b".to_string());
        list.push("c".to_string());

        // for v in 

    }
}
