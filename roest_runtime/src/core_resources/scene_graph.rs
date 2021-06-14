use legion::Entity;
use std::fmt;
use std::collections::HashMap;
use serde::de::{DeserializeSeed, Visitor, SeqAccess, Error};
use serde::{Deserializer, Serialize, Serializer};
use serde::ser::SerializeStruct;
use slotted_tree::{Tree, TreeKey, TreeError};


pub struct DeMap {
    entity_map: HashMap<uuid::Bytes, Entity>
}

pub struct SerSceneGraph {
    scene_graph: SceneGraph,
    entity_map: HashMap<Entity, uuid::Bytes>
}

pub struct SceneGraph {
    tree: Tree<Entity>,
}

impl SceneGraph {
    pub fn new(root_entity: Entity) -> Result<Self, TreeError> {
        let tree: Tree<Entity> = Tree::new(root_entity);

        Ok(SceneGraph {
            tree
        })
    }

    pub fn add_child(&mut self, parent: TreeKey, child: Entity) -> Result<TreeKey, TreeError> {
        self.tree.add_child(parent, child)
    }

    pub fn tree(&self) -> &Tree<Entity> {
        &self.tree
    }

    pub fn tree_mut(&mut self) -> &mut Tree<Entity> {
        &mut self.tree
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
                let uuid_tree: Tree<uuid::Bytes> = seq.next_element()?
                    .ok_or_else(|| Error::invalid_length(1, &self))?;


                let tree = uuid_tree.map(
                    |node| self.0.entity_map.get(node).unwrap().clone()
                );

                Ok(SceneGraph { tree })
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
            |node| self.entity_map.get(node).unwrap().clone(),
        );

        ser.serialize_field("graph", &uuid_graph)?;
        ser.end()
    }
}