use std::ops::Index;
use serde::{Serialize, Deserialize};
use failure::Fail;
use gl_renderer::data::matrix_data::mat3;

// TODO: generational indices to prevent invalid TreeIndex handles from being used?
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct TreeIndex(u32);

#[derive(Serialize, Deserialize)]
pub struct Tree<T> {
    vec: Vec<Option<TreeNode<T>>>,
    empty_spots: Vec<TreeIndex>,
}

#[derive(Debug, Fail)]
pub enum TreeError {
    #[fail(display = "Tree is out of empty space")]
    OutOfSpace,
    #[fail(display = "Parent does not exist")]
    InvalidParent,
}

impl<T> Tree<T> {
    pub fn add_orphan(&mut self, orphan: TreeNode<T>) -> Result<TreeIndex, TreeError> {
        let idx = self.empty_spots.pop().ok_or(TreeError::OutOfSpace)?;
        self.vec[idx.0 as usize] = Some(orphan);

        Ok(idx)
    }

    pub fn add_child(&mut self, parent: TreeIndex, child: TreeNode<T>) -> Result<TreeIndex, TreeError> {
        let idx = self.empty_spots.pop().ok_or(TreeError::OutOfSpace)?;

        match &mut self.vec[parent.0 as usize] {
            Some(ref mut p) => { p.add_child(idx) },
            None => { return Err(TreeError::InvalidParent) }
        }

        self.vec[idx.0 as usize] = Some(child);

        Ok(idx)
    }

    // pub fn add_children(&mut self, parent: TreeIndex, children: Vec<TreeNode<T>>) -> Result<Vec<TreeIndex>, TreeError> {
    //     let mut indices = Vec::with_capacity(children.len());
    //
    //     for i in 0..indices.len() {
    //         let idx = self.empty_spots.pop().ok_or(TreeError::OutOfSpace)?;
    //         indices[i] = idx;
    //         self.vec[idx.0 as usize] = Some(children.remove(i))
    //     }
    //
    //     match &self.vec[parent.0 as usize] {
    //         Some(mut p) => { p.add_children(&indices) },
    //         None => { return Err(TreeError::InvalidParent) }
    //     }
    //     Ok(indices)
    // }

    pub fn remove_tree(&mut self, node: TreeIndex) {
        let tree = match &self.vec[node.0 as usize] {
            Some(t) => {
                t.children.clone()
            }
            None => return
        };

        for t in tree {
            self.remove_tree(t);
        }

        self.vec[node.0 as usize] = None;
        self.empty_spots.push(node);
    }

    pub fn map<T2, F>(&self, mut map_func: F) -> Tree<T2>
        where F: FnMut(TreeIndex, &T) -> T2 {
        let mut tree = Tree {
            vec: Vec::with_capacity(self.vec.len()),
            empty_spots: self.empty_spots.clone()
        };

        for (idx, node) in self.vec.iter().enumerate() {
            match node {
                Some(n) => {
                    let new_val = map_func(TreeIndex(idx as u32), &n.val);
                    tree.vec[idx] = Some(TreeNode::new(new_val))
                }
                None => tree.vec[idx] = None
            }
        }

        tree
    }
}

impl<T> Default for Tree<T> {
    fn default() -> Self {
        let default_size: u32 = 256;
        let empty_spots = {
            let mut vec = Vec::with_capacity(default_size as usize);

            for i in 0..default_size {
                vec.push(TreeIndex(i));
            }

            vec
        };

        // let vec = vec![None; default_size as usize];
        let mut vec = Vec::with_capacity(default_size as usize);

        for i in 0..default_size {
            vec.push(None);
        }

        Tree {
            vec,
            empty_spots,
        }
    }
}

impl<T> Index<TreeIndex> for Tree<T> {
    type Output = Option<TreeNode<T>>;

    fn index(&self, index: TreeIndex) -> &Self::Output {
        &self.vec[index.0 as usize]
    }
}

#[derive(Serialize, Deserialize)]
pub struct TreeNode<T> {
    children: Vec<TreeIndex>,
    val: T,
}

impl<T> TreeNode<T> {
    pub fn new(val: T) -> Self {
        TreeNode {
            children: Vec::new(),
            val
        }
    }

    pub fn with_children(val: T, children: &[TreeIndex]) -> Self {
        TreeNode {
            children: Vec::from(children),
            val
        }
    }

    // NOTE: maybe this method is redundant?
    pub fn add_child(&mut self, child: TreeIndex) {
        self.children.push(child);
    }

    pub fn add_children(&mut self, children: &[TreeIndex]) {
        self.children.extend_from_slice(children)
    }

    pub fn get_children(&self) -> &[TreeIndex] {
        &self.children[..]
    }

    pub fn get_val(&self) -> &T {
        &self.val
    }
}
