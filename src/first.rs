/* linked list란? a bunch of pieces of data on the heap
 * that point to each other
 */

use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
    // Box<T> is a smart pointer to a heap allocated T
    // Use Box<T> to create a recursive data structure
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
    
    // push a new element at the front of the list
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
            // mem::replace: Moves src into the referenced dest, returning the previous dest value.
        });

        self.head = Link::More(new_node);
    }

    // pop the front-most element
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        // Stack overflow를 막기 위해 재귀 대신 while loop 사용
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // cur_link가 Link::More인 경우만 while loop 실행
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
    }
}
