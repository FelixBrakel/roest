use crate::core_systems::object_models;

struct StaticObject {
    mesh_filepath: String,
}

impl object_models::game_object::GameObject for StaticObject {

}

impl object_models::mesh::Mesh for StaticObject {

}
