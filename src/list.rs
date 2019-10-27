use std::mem;

pub struct List<T> {
    head: Option<Box<Node<T>>>,
    size: i32,
}

pub enum Node<T> {
    leaf(T), 
    branch(T, Box<Node<T>>),
}

impl <T> List<T> {
    use Node::branch;
    pub fn new() -> Self {
        List { head: None, size: 0 }
    }

    pub fn push(&mut self, elem: T) {
        // TODO properly handle the case where
        let new_node = branch(elem, mem::replace(&mut self.head, None));
        self.head = Some(new_node);
        self.size = self.size + 1;
    }

    // pub fn pop(&mut self) -> Option<T> {
    //     match mem::replace(&mut self.head, None) {
    //         None => None,
    //         Some(x) => {
    //             self.head = x.next;
    //             self.size = self.size - 1;
    //             Some(x.elem)
    //         }
    //     }
    // }

    // pub fn peek(&mut self) -> Option<&T> {
    //     match &self.head {
    //         None => None,
    //         Some(x) => Some(&x.elem)
    //     }
    // }

    pub fn size(&mut self) -> i32 {
        self.size
    }

}

// impl <T> Drop for List<T> {
//     fn drop(&mut self) {
//         let mut cur_link = mem::replace(&mut self.head, None);

//         while let Some(mut boxed_node) = cur_link {
//             cur_link = mem::replace(&mut boxed_node.next, None);
//         }
//     }
// }


#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::<i32>::new();

        // assert_eq!(list.pop(), None);
        // assert_eq!(list.peek(), None);
        assert_eq!(list.size(), 0);

        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.size(), 3);

        // assert_eq!(list.peek(), Some(&3));
        // assert_eq!(list.pop(), Some(3));
        // assert_eq!(list.pop(), Some(2));
        // assert_eq!(list.size(), 1);

        // list.push(4);
        // list.push(5);
        // assert_eq!(list.size(), 3);

        // assert_eq!(list.peek(), Some(&5));
        // assert_eq!(list.pop(), Some(5));
        // assert_eq!(list.pop(), Some(4));
        // assert_eq!(list.pop(), Some(1));
        // assert_eq!(list.pop(), None);
        // assert_eq!(list.size(), 0);

    }
}
