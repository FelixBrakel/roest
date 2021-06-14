use serde::{Serialize, Deserialize};
use thiserror::Error;
use slotmap::{SlotMap, SecondaryMap, new_key_type};

new_key_type! {
    pub struct TreeKey;
}

#[derive(Serialize, Deserialize)]
pub struct TreeNode<T> {
    children: Vec<TreeKey>,
    val: T,
}

impl<T> TreeNode<T> {
    pub fn new(val: T) -> Self {
        TreeNode {
            children: Vec::new(),
            val
        }
    }

    pub fn with_children(val: T, children: &[TreeKey]) -> Self {
        TreeNode {
            children: Vec::from(children),
            val
        }
    }

    // NOTE: maybe this method is redundant?
    pub fn add_child(&mut self, child: TreeKey) {
        self.children.push(child);
    }

    pub fn add_children(&mut self, children: &[TreeKey]) {
        self.children.extend_from_slice(children)
    }

    pub fn get_children(&self) -> &[TreeKey] {
        &self.children[..]
    }

    pub fn get_val(&self) -> &T {
        &self.val
    }
}

pub struct SecondaryTree<T>(SecondaryMap<TreeKey, T>);

impl<T> SecondaryTree<T> {
    pub fn new() -> Self {
        SecondaryTree(SecondaryMap::new())
    }

    pub fn get(&self, node: TreeKey) -> Option<&T> {
        self.0.get(node)
    }

    pub fn insert(&mut self, node: TreeKey, value: T) {
        self.0.insert(node, value);
    }
}

#[derive(Debug, Error)]
pub enum TreeError {
    #[error("Tree is out of empty space")]
    OutOfSpace,
    #[error("Parent does not exist")]
    InvalidParent,
}

#[derive(Serialize, Deserialize)]
pub struct Tree<T> {
    root: TreeKey,
    nodes: SlotMap<TreeKey, TreeNode<T>>,
}

impl<T> Tree<T> {
    pub fn new(root: T) -> Self {
        let mut nodes = SlotMap::with_key();
        let r = nodes.insert(TreeNode::new(root));

        Tree {
            root: r,
            nodes,
        }
    }

    pub fn add_child(&mut self, parent: TreeKey, child: T) -> Result<TreeKey, TreeError> {
        if !self.nodes.contains_key(parent) {
            return Err(TreeError::InvalidParent)
        }
        let new_key = self.nodes.insert(TreeNode::new(child));

        let node = self.nodes.get_mut(parent).unwrap();
        node.children.push(new_key);
        Ok(new_key)
    }

    pub fn remove_tree(&mut self, key: TreeKey) -> Vec<T> {
        let mut node = self.nodes.remove(key);

        let mut out = Vec::new();

        let mut children = Vec::new();
        while let Some(n) = node {
            out.push(n.val);
            children.extend(n.children);
            let child = children.pop();
            node = match child {
                Some(ch) => self.nodes.remove(ch),
                None => None
            };
        }

        return out;
    }

    /// Attach a secondary data storage to the tree. Keys from the original are still valid
    pub fn map_to_secondary<T2, F>(&self, mut map_func: F) -> SecondaryTree<T2>
        where F: FnMut(&T) -> T2 {
        let mut tree = SecondaryTree ( SecondaryMap::new() );

        for k in self.nodes.keys() {
            let v = self.nodes.get(k).unwrap();
            let new_val = map_func(&v.val);
            tree.0.insert(k, new_val);
        }

        tree
    }

    /// Maps the tree to a new tree, here the keys from the current tree are not valid.
    pub fn map<T2: Copy, F>(&self, mut map_func: F) -> Tree<T2>
        where F: FnMut(&T) -> T2
    {
        let root = &self.nodes.get(self.root).unwrap().val;
        let mut tree = Tree::new(map_func(root));

        self.map_children_rec(&mut tree, self.root, &mut map_func);

        tree
    }

    fn map_children_rec<T2: Copy, F>(&self, tree: &mut Tree<T2>, node: TreeKey, map_func: &mut F)
        where F: FnMut(&T) -> T2
    {
        let n = self.nodes.get(node).unwrap();
        for c in &n.children {
            tree.add_child(node, map_func(self.value(*c).unwrap())).unwrap();
            self.map_children_rec(tree, *c, map_func)
        }
    }

    pub fn children(&self, parent: TreeKey) -> Option<&[TreeKey]> {
        let node = self.nodes.get(parent)?;

        Some(&node.children)
    }

    pub fn value(&self, node: TreeKey) -> Option<&T> {
        self.nodes.get(node).map(|n| &n.val)
    }

    pub fn root(&self) -> TreeKey {
        self.root
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
