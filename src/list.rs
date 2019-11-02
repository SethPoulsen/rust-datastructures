
pub enum List<T> {
    Nil, 
    Cons(Box<T>, Box<List<T>>),
}

impl <T> List<T> {

    pub fn new() -> Self {
        List::Nil
    }

    pub fn push(self, data: T) -> List<T> {
        List::Cons(Box::new(data), Box::new(self))
    }

    pub fn peek(&self) -> Option<&T> {
        match &self {
            List::Nil => None,
            List::Cons(data, _) => Some(&data)
        }
    }

    pub fn pop(self) -> (List<T>, Option<T>) {
        match self {
            List::Nil => (List::Nil, None),
            List::Cons(data, next) => (*next, Some(*data))
        }
    }

    pub fn size(&self) -> i32 {
        match &self {
            List::Nil => 0,
            List::Cons(_, next) => 1 + next.size()
        }
    }

}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::<i32>::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.size(), 0);

        list = list.push(1);
        list = list.push(2);
        list = list.push(3);
        assert_eq!(list.size(), 3);

        assert_eq!(list.peek(), Some(&3));

        let mut temp = list.pop(); assert_eq!(temp.1, Some(3));
        temp = temp.0.pop(); assert_eq!(temp.1, Some(2));

        list = temp.0.push(4);
        list = list.push(5);
        assert_eq!(list.size(), 3);

        assert_eq!(list.peek(), Some(&5));
        temp = list.pop(); assert_eq!(temp.1, Some(5));
        temp = temp.0.pop(); assert_eq!(temp.1, Some(4));
        temp = temp.0.pop(); assert_eq!(temp.1, Some(1));
        temp = temp.0.pop(); assert_eq!(temp.1, None);

        assert_eq!(temp.0.size(), 0);

    }
}
