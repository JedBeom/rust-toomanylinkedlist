/* linked list란? a bunch of pieces of data on the heap
 * that point to each other
 */

pub struct List<T> {
    head: Link<T>,
}

// type alias
type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

// 제네릭을 위해서라면 impl 뒤에도 <T>가 붙어야 한다
impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }
    
    // push a new element at the front of the list
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    // pop the front-most element
    pub fn pop(&mut self) -> Option<T> {
        // map은 Some(X)를 Some(Y)로, None은 None으로 반환
        self.head.take().map( |node| {
            self.head = node.next;
            node.elem
        })
    }
}

// 제네릭을 위해서라면 impl 뒤에도 <T>가 붙어야 한다
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // Stack overflow를 막기 위해 재귀 대신 while loop 사용
        let mut cur_link = self.head.take();
        // cur_link가 Link::More인 경우만 while loop 실행
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
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
