use std::mem;

pub struct List<T> {
    head: Box<Node<T>>,
    size: i32,
}

enum Node<T> {
    Nil, 
    Cons(T, Box<Node<T>>),
}

impl <T> List<T> {
    pub fn new() -> Self {
        List { head: Box::new(Node::Nil), size: 0 }
    }

    pub fn push(&mut self, data: T)  {
        let temp = mem::replace(&mut *self.head, Node::Nil);
        *self.head = Node::Cons(data, Box::new(temp));
        self.size = self.size + 1;
    }

    pub fn peek(&self) -> Option<&T> {
        match &*self.head {
            Node::Nil => None,
            Node::Cons(data, _) => Some(&data)
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match mem::replace(&mut *self.head, Node::Nil) {
            Node::Nil => None, 
            Node::Cons(data, next) => {
                self.head = next;
                self.size = self.size - 1;
                Some(data)
            }
        }
    }

    pub fn size(&self) -> i32 {
        self.size
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::<i32>::new();

        assert_eq!(list.pop(), None);
        assert_eq!(list.peek(), None);
        assert_eq!(list.size(), 0);

        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.size(), 3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.size(), 1);

        list.push(4);
        list.push(5);
        assert_eq!(list.size(), 3);

        assert_eq!(list.peek(), Some(&5));
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
        assert_eq!(list.size(), 0);

    }
}
