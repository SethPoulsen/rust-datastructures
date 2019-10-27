use std::mem;

type Link = Option<Box<Node>>;

pub struct BST {
    head: Link,
}

struct Node {
    elem: i32,
    left: Link,
    right: Link,
}

impl BST {
    pub fn new() -> Self {
        BST { head: None }
    }

    // pub fn insert(&mut self, elem: i32) -> bool {
    //     let new_node = Box::new(Node {
    //         elem: elem, 
    //         left: None, 
    //         right: None,
    //     });
    //     match &mut self.head {
    //         None => {
    //             mem::replace(&mut self.head, Some(new_node));
    //             true
    //         }
    //         Some(x) => self.recursive_insert(self.head, Some(new_node)),
    //     }
    // }

    fn recursive_insert(&self, cur_head: Link, new_node: Link) -> bool {
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

        assert_eq!(true, true);

    }
}