/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        let mut node = TreeNode::new(value);
        let mut root = &mut self.root;
        if root.is_none() {
            self.root = Some(Box::new(node));
            return ;
        }

        while let Some(ref mut x) = root {
            if node.value < x.value {
                match &x.left {
                    Some (y) => {
                        root = &mut x.left;
                    }
                    None => {
                        x.left = Some(Box::new(node));
                        return;
                    }
                }
            } else if node.value > x.value {
                match &x.right {
                    Some(y) => {
                        root = &mut x.right;
                    }
                    None => {
                        x.right = Some(Box::new(node));
                        return;
                    }
                }
            } else {
                return;
            }

        }



    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        if self.root.is_none() {
            return false;
        }
        let mut root = &self.root;
        while  let Some(x) = root {
            if x.value == value {
                return true;
            }

            if value < x.value {
                root = &x.left;
            } else {
                root = &x.right;
            }
        }
        false
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


