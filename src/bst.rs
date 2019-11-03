// use std::mem;

pub struct BST {
    head: Box<Node>,
}

enum Node {
    Nil,
    Branch(i32, Box<Node>, Box<Node>)
}



impl BST {
    pub fn new() -> Self {
        BST { head: Box::new(Node::Nil) }
    }

    pub fn insert(&mut self, data: i32) -> bool {
        match &mut *self.head {
            Node::Nil => {
                *self.head = Node::Branch(data, Box::new(Node::Nil), Box::new(Node::Nil));
                true
            }
            Node::Branch(_,_,_) => BST::recursive_insert(&mut self.head, data)
        }
    }

    fn recursive_insert(_cur_head: &mut Box<Node>, _data: i32) -> bool {
        false
    }

    // pub fn get(elem: i32) -> Option<&i32> {
        
    // }

}


#[cfg(test)]
mod test {
    use super::BST;

    #[test]
    fn basics() {
        let mut bst = BST::new();
        bst.insert(50);

        assert_eq!(true, true);

    }
}