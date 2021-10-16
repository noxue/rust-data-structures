use std::{cell::RefCell, rc::Rc};

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
        data.clone().unwrap().borrow_mut().next = self.head.take();
        self.head = data;
        self.len += 1;
    }
}



#[cfg(test)]
mod tests{
    use std::borrow::Borrow;

    use super::LinkedList;


    #[test]
    fn list_new(){
        let mut list = LinkedList::new();
        list.push("a".to_string());
        list.push("b".to_string());
        list.push("c".to_string());

        let mut node = list.head;
        while let Some(v) = node{
            
            println!("{}", v.clone().borrow_mut().data);
            node = v.clone().borrow_mut().next.clone();
        }
    }
    
}