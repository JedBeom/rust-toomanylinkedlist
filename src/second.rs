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

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    // mutable version
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
}

// 제네릭을 위해서라면 impl 뒤에도 <T>가 붙어야 한다
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // Stack overflow를 막기 위해 재귀 대신 while loop 사용
        let mut cur_link = self.head.take();
        // cur_link가 Link::Some인 경우만 while loop 실행
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take()
        }
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
        self.0.pop() // IntoIter가 tuple struct이기 때문에 0으로 접근
    }
}

// Iter
/* lifetime is required here
 * lifetime is the name of a region(block/scope), related to 'how long do the references live'
 * Rust has syntactic sugar for lifetimes:
 *      Only one reference in input, and the output is derived from the input
 *      Many inputs and no outputs: assume that they're all independent
 *      Methods: assume all output lifetimes are derived from `self` (other inputs are assumed to
 *          have independent lifetimes
 *      We need to add lifetimes only in function and type signatures.
 */
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

// List does not have any associated lifetimes (We want lifetimes for `Iter`)
impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_deref() } 
        // 1. `.as_deref()`는 `.as_ref().map(|node| &**node)`와 같은 동작을 한다.
        //      Option<U>를 Option<&(U.deref())>로 바꾼다.
        //      그래서 self.head의 Option 안은 Box<Node<T>>인데 as_deref가 &Node<T>로 바꿔버리는 것
        // 2. Box는 Deref Trait이 있기 때문에, `*` 연산자의 작동을 커스터마이징 해놓았다.
        //      그래서 Box는 reference가 아님에도 일반 reference처럼 dereference된다.
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T; // type declaration이기 때문에 lifetime 필요
                       
    fn next(&mut self) -> Option<Self::Item> { // `type Item ... ` 줄에서 lifetime이 지정되었기
                                               // 때문에 여긴 필요없음
        self.next.map(|node| {
            self.next = node.next.as_deref();
            // self.next = node.next.as_ref().map::<&Node<T>, _>(|node| &node);
            // ...로도 쓸 수 있다. 
            // ::<> (turbofish 문법이라고 이름 붙여짐)는 컴파일러에게 제네릭이 가져야하는 타입을
            // 확실하게 지정한다.
            &node.elem
        })
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

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        // change the value
        // the solution from ChatGPT
        let mut value = list.peek_mut();
        if let Some(ref_mut) = &mut value {
            **ref_mut = 5;
        }

        assert_eq!(list.peek(), Some(&5));

        // change the value
        // from the text
        list.peek_mut().map(|value| {
            *value = 42;
        });

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter(); // 이미 ownership 옮겨짐 
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
}
