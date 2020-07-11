use legion::storage::Component;

pub struct ComponentRepresentation {
    name: String
}

impl ComponentRepresentation {
    pub fn new(name: String) -> ComponentRepresentation {
        ComponentRepresentation {
            name
        }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

pub trait EditorComponent: Component {
    fn as_component_representation(&self) -> ComponentRepresentation;
}
