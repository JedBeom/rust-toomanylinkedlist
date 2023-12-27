use std::rc::Rc;
use std::cell::{Ref, RefCell};

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem,
            prev: None,
            next: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: None }
    }

    pub fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);
        match self.head.take() { // 소유권 겟또
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => { // empty list
                self.tail = Some(new_head.clone()); // 이건 Reference Count 증가?
                self.head = Some(new_head); // 이건 move?
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => { // not empty list
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => { // empty list
                      self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
            /*
             * try_unwrap: 앞에서 썼듯이, reference count가 정확히 1이면 unwrap
             *      제대로 짰으면 unwrap이 성공해야함
             * ok: Result -> Option 변환 (consume)
             * unwrap: Option이 Some이라는 가정 하에 unwrap (consume)
             * into_inner: RefCell을 벗겨내기 (consume)
             */
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> { 
        // Ref는 일단 갖고 나가야 함...
        // Ref를 벗겨서 나갈 수가 없다
        self.head.as_ref().map(|node| {
            Ref::map(node.borrow(), |node| &node.elem)
            // Ref::map: Make a new Ref for a component of the borrowed data
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {} // None이 나올 때까지 pop
    }
}


#[cfg(test)]
mod test {
    use super::List;
    
    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop_front(), None);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        list.push_front(4);
        list.push_front(5);

        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(*list.peek_front().unwrap(), 3);
        // list.peek_front(): Option<Ref<T>>
        // list.peek_front().unwrap(): Ref<T>
        // *list.peek_front().unwrap(): T
    }
} 
