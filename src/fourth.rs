use std::rc::Rc;
use std::cell::{Ref, RefCell, RefMut};

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
             *      fourth List를 제대로 짰으면 unwrap이 성공해야함
             * ok: Result -> Option 변환 (consume)
             * unwrap: Option이 Some이라는 가정 하에 unwrap (consume)
             * into_inner: RefCell을 벗겨내기 (consume)
             */
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> { 
        // Ref는 일단 갖고 나가야 함...
        // Ref를 벗기면 이 메서드 scope가 끝나면서 temporary value가 drop됨
        self.head.as_ref().map(|node| {
            Ref::map(node.borrow(), |node| &node.elem)
            // Ref::map: Make a new Ref for a component of the borrowed data
        })
    }

    // 이하는 위 메서드들을 복붙한다음 front -> back 등으로 단어 교체한 것들
    
    pub fn push_back(&mut self, elem: T) {
        let new_tail = Node::new(elem);
        match self.tail.take() { 
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => { 
                self.tail = Some(new_tail.clone()); 
                self.head = Some(new_tail); 
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => { // not empty list
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => { // empty list
                      self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }

    pub fn peek_back(&self) -> Option<Ref<T>> { 
        self.tail.as_ref().map(|node| {
            Ref::map(node.borrow(), |node| &node.elem)
        })
    }

    // peek mut ver.
    // Ref -> RefMut로 바뀜

    pub fn peek_front_mut(&self) -> Option<RefMut<T>> { 
        self.head.as_ref().map(|node| {
            RefMut::map(node.borrow_mut(), |node| &mut node.elem)
        })
    }

    pub fn peek_back_mut(&self) -> Option<RefMut<T>> { 
        self.tail.as_ref().map(|node| {
            RefMut::map(node.borrow_mut(), |node| &mut node.elem)
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {} // None이 나올 때까지 pop
    }
}

// IntoIter
pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

// Iter
// 있었는데요... 없었습니다.
// RefCell의 농간에 그만...
// Rc를 반환하자니 더 이상 borrow가 아니라는 문제가...

#[cfg(test)]
mod test {
    use super::List;
    
    #[test]
    fn basics() {
        let mut list = List::new();

        // ---- front ----
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

        // ---- back ----
        assert_eq!(list.pop_back(), None);

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        list.push_back(4);
        list.push_back(5);

        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));

        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());
        assert!(list.peek_front_mut().is_none());
        assert!(list.peek_back_mut().is_none());

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(*list.peek_front().unwrap(), 3);
        // list.peek_front(): Option<Ref<T>>
        // list.peek_front().unwrap(): Ref<T>
        // *list.peek_front().unwrap(): T
        
        {
            let mut a = list.peek_front_mut().unwrap();
            *a = 4;
        }

        assert_eq!(*list.peek_front().unwrap(), 4);

        assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 4);
        assert_eq!(&*list.peek_back().unwrap(), &1);
        assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 1);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }
} 
