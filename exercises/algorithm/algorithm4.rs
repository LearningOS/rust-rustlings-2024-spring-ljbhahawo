/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
#[derive(PartialEq)]
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
        //TODO
        // 对self.root进行模式匹配
        // root是一个option，如果为none，则new一个返回
        // 如果是some，取得其中node的可变引用，然后调用node的insert进行插入
        match self.root{
            None=>self.root = Some(Box::new(TreeNode::new(value))),
            Some(ref mut node) => node.insert(value),
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        // 对self.root进行模式匹配
        // root是一个option，如果为none，则说明值不存在
        // 如果是some，取得其中node的可变引用，然后调用node的search进行查找
        match self.root{
            None=>false,
            Some(ref node) => node.search(value),
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        // 如果值相等，则什么都不用做
        // 如果结点上的val 小于 插入的val，说明要插在右子树，如果右子树为空则直接插入，否则递归插入
        // 如果结点上的val 大于 插入的val，说明要插在左子树，如果左子树为空则直接插入，否则递归插入
        if self.value < value{
            match self.right {
                None => self.right = Some(Box::new(TreeNode::new(value))),
                Some(ref mut node) => node.insert(value),
            }
        }else if self.value > value {
            match self.left {
                None => self.left = Some(Box::new(TreeNode::new(value))),
                Some(ref mut node) => node.insert(value),
            }
        }
    }

    fn search(&self,value: T)->bool{
        if self.value == value{
            true
        }else if self.value > value {
            match self.left {
                None => false,
                Some(ref node) => node.search(value),
            }
        }else {
            match self.right {
                None => false,
                Some(ref node) => node.search(value),
            }
        }
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


