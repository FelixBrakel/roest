use legion::entity::Entity;
use petgraph::{Graph};
use std::fmt;
use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use serde::de::{self, DeserializeSeed, Visitor, SeqAccess, Error};
use serde::{Deserializer, Serialize, Serializer};
use serde::ser::SerializeStruct;
use super::tree::{TreeIndex, TreeError, Tree, TreeNode};

pub struct DeMap {
    entity_map: HashMap<uuid::Bytes, Entity>
}

pub struct SerSceneGraph {
    scene_graph: SceneGraph,
    entity_map: HashMap<Entity, uuid::Bytes>
}

pub struct SceneGraph {
    root: TreeIndex,
    tree: Tree<Entity>,
    listeners: Vec<dyn >
}

impl SceneGraph {
    pub fn new(root_entity: Entity) -> Result<Self, TreeError> {
        let mut tree: Tree<Entity> = Default::default();
        let root = tree.add_orphan(TreeNode::new(root_entity))?;

        Ok(SceneGraph {
            root,
            tree
        })
    }

    pub fn add_child(&mut self, parent: TreeIndex, child: Entity) -> Result<TreeIndex, TreeError> {
        self.tree.add_child(parent, TreeNode::new(child))
    }

    pub fn root(&self) -> TreeIndex {
        self.root
    }

    pub fn tree(&self) -> &Tree<Entity> {
        &self.tree
    }
}

impl Index<TreeIndex> for SceneGraph {
    type Output = Entity;

    fn index(&self, node: TreeIndex) -> &Entity {
        let node = &self.tree[node];

        let node = match node {
            Some(n) => {
                n
            }
            None => panic!("Node at index does not exist")
        };

        node.get_val()
    }
}

impl<'de> DeserializeSeed<'de> for DeMap {
    type Value = SceneGraph;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
        struct SceneGraphVisitor<'a>(&'a DeMap);

        impl<'a, 'de> Visitor<'de> for SceneGraphVisitor<'a> {
            type Value = SceneGraph;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a SceneGraph struct")
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let root = seq.next_element()?
                    .ok_or_else(|| Error::invalid_length(0, &self))?;
                let uuid_tree: Tree<uuid::Bytes> = seq.next_element()?
                    .ok_or_else(|| Error::invalid_length(1, &self))?;


                let tree = uuid_tree.map(
                    |_, node| self.0.entity_map.get(node).unwrap().clone()
                );

                Ok(SceneGraph { root, tree })
            }

            //TODO: support more serializers by implementing visit_map
        }

        deserializer.deserialize_struct("SceneGraph", &["graph"], SceneGraphVisitor(&self))
    }
}

impl Serialize for SerSceneGraph {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut ser = serializer.serialize_struct("SceneGraph", 2)?;
        let uuid_graph  = self.scene_graph.tree.map(
            |_, node| self.entity_map.get(node).unwrap().clone(),
        );

        ser.serialize_field("root", &self.scene_graph.root);
        ser.serialize_field("graph", &uuid_graph);
        ser.end()
    }
}