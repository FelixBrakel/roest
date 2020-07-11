use serde::{Serialize, Deserialize};
use thiserror::Error;
use slotmap::{SlotMap, SecondaryMap, new_key_type};

new_key_type! {
    pub struct TreeKey;
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

#[derive(Serialize, Deserialize)]
pub struct Tree<T: Copy> {
    root: TreeKey,
    values: SlotMap<TreeKey, T>,
    // NOTE: The Vec<TreeKey> does not implement Copy so it has to be in a separate SecondaryMap, this can be changed
    //       once Copy is no longer a supertrait for Slottable
    children: SecondaryMap<TreeKey, Vec<TreeKey>>
}

#[derive(Debug, Error)]
pub enum TreeError {
    #[error("Tree is out of empty space")]
    OutOfSpace,
    #[error("Parent does not exist")]
    InvalidParent,
}

impl<T: Copy> Tree<T> {
    pub fn new(root: T) -> Self {
        let mut values = SlotMap::with_key();
        let r = values.insert(root);
        let mut secondary = SecondaryMap::new();
        secondary.insert(r, Vec::new());

        Tree {
            root: r,
            values,
            children: secondary
        }
    }

    pub fn add_child(&mut self, parent: TreeKey, child: T) -> Result<TreeKey, TreeError> {
        let idx = match self.children.get_mut(parent) {
            Some(p) => {
                let i = self.values.insert(child);
                p.push(i);
                self.children.insert(i, Vec::new());
                i
            },
            None => { return Err(TreeError::InvalidParent) }
        };

        Ok(idx)
    }

    pub fn remove_tree(&mut self, node: TreeKey) {
        let tree = match self.children.get(node) {
            Some(t) => t.clone(),
            None => return
        };

        for t in tree {
            self.remove_tree(t);
        }

        self.values.remove(node);
        self.children.remove(node);
    }

    /// Maps the tree to a SecondaryTree in which all the keys from the current tree are still valid
    pub fn map_to_secondary<T2, F>(&self, mut map_func: F) -> SecondaryTree<T2>
        where F: FnMut(TreeKey, &T) -> T2 {
        let mut tree = SecondaryTree ( SecondaryMap::new() );

        for k in self.values.keys() {
            let v = self.values.get(k).unwrap();
            let new_val = map_func(k, v);
            tree.0.insert(k, new_val);
        }

        tree
    }

    /// Maps the tree to a new tree, here the keys from the current tree are not valid.
    pub fn map<T2: Copy, F>(&self, mut map_func: F) -> Tree<T2>
        where F: FnMut(TreeKey, &T) -> T2
    {
        let root = self.values.get(self.root).unwrap();
        let mut tree = Tree::new(map_func(self.root, root));

        self.map_children_rec(&mut tree, self.root, &mut map_func);

        tree
    }

    fn map_children_rec<T2: Copy, F>(&self, tree: &mut Tree<T2>, node: TreeKey, map_func: &mut F)
        where F: FnMut(TreeKey, &T) -> T2
    {
        let ch = self.children(node).unwrap();
        for c in ch {
            tree.add_child(node, map_func(*c, self.value(*c).unwrap())).unwrap();
            self.map_children_rec(tree, *c, map_func)
        }
    }

    pub fn children(&self, parent: TreeKey) -> Option<&[TreeKey]> {
        match self.children.get(parent) {
            Some(c) => Some(c),
            None => None
        }
    }

    pub fn value(&self, node: TreeKey) -> Option<&T> {
        self.values.get(node)
    }

    pub fn root(&self) -> TreeKey {
        self.root
    }
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
