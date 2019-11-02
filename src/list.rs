use std::mem;

// #[derive(Copy, Clone)] // more advanced, maybe for starters just copy it?
pub enum List<T> {
    Nothing, 
    Cons(Box<T>, Box<List<T>>),
}

impl <T> List<T> {

    pub fn new() -> Self {
        List::Nothing
    }

    // pub fn push(&mut self, data: T) -> List<T> {
    //     List::Cons(Box::new(data), Box::new(match &self))
    // }

    // pub fn peek(&self) -> Option<&T> {
    //     match &self {
    //         List::Nothing => None,
    //         List::Cons(data, next) => Some(&data)
    //     }
    // }

    pub fn size(&self) -> i32 {
        match &self {
            List::Nothing => 0,
            List::Cons(data, next) => 1 + next.size()
        }
    }

}

// impl <T> Drop for List<T> {
//     fn drop(&mut self) {
//         let mut cur_link = mem::replace(&mut self.head, None);

//         while let Some(mut boxed_node) = cur_link {
//             cur_link = match mem::replace(&mut boxed_node.next, None);
//         }
//     }
// }


#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::<i32>::new();

        // assert_eq!(None, match &list {
        //     Nothing => None,
        //     List::Cons(data, next) => Some(data)
        // });
        // assert_eq!(list.peek(), None);
        assert_eq!(list.size(), 0);

        // list = List::Cons(1, Box::new(list));
        // list = List::Cons(2, Box::new(list));
        // list = List::Cons(3, Box::new(list));

        // assert_eq!(list.size(), 3);

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
