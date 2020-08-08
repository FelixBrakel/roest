use legion::{
    storage::{Component},
    prelude::{
        World,
        Entity
    }
};

use gtk::Paned;
use glib::bitflags::_core::cell::RefCell;
use std::rc::Rc;
use crate::EditorWorld;

pub struct ComponentRepresentation {
    name: String,
    insert_func: fn(&mut World, Entity),
    ui_func: fn(Rc<RefCell<EditorWorld>>, Entity, &Paned)
}

impl ComponentRepresentation {
    pub fn new(name: String, insert_func: fn(&mut World, Entity), ui_func: fn(Rc<RefCell<EditorWorld>>, Entity, &Paned)) -> ComponentRepresentation {
        ComponentRepresentation {
            name,
            insert_func,
            ui_func
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn ui_func(&self) -> fn(Rc<RefCell<EditorWorld>>, Entity, &Paned) {
        self.ui_func
    }
}

pub trait EditorComponent: Component {
    fn as_component_representation(&self) -> ComponentRepresentation;
    // fn insert_func() -> Box<dyn Fn(World, Entity)>;
    // fn attach_ui(&self, paned: &gtk::Paned);
}
