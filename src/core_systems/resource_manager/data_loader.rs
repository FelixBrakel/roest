use std::path::Path;
use tobj;

pub fn load_obj_file(filepath: &str) -> Vec<tobj::Model> {

    let cornell_box = tobj::load_obj(&Path::new(filepath));
    assert!(cornell_box.is_ok());
    let (models, materials) = cornell_box.unwrap();

    println!("# of models: {}", models.len());
    println!("# of materials: {}", materials.len());
//    for (i, m) in models.iter().enumerate() {
//        let mesh = &m.mesh;
//        println!("model[{}].name = \'{}\'", i, m.name);
//        println!("model[{}].mesh.material_id = {:?}", i, mesh.material_id);
//
//        println!("Size of model[{}].indices: {}", i, mesh.indices.len());
//        for f in 0..mesh.indices.len() / 3 {
//            println!("    idx[{}] = {}, {}, {}.", f, mesh.indices[3 * f],
//                     mesh.indices[3 * f + 1], mesh.indices[3 * f + 2]);
//        }
//
//         Normals and texture coordinates are also loaded, but not printed in this example
//        println!("model[{}].vertices: {}", i, mesh.positions.len() / 3);
//        assert!(mesh.positions.len() % 3 == 0);
//        for v in 0..mesh.positions.len() / 3 {
//            println!("    v[{}] = ({}, {}, {})", v, mesh.positions[3 * v],
//                     mesh.positions[3 * v + 1], mesh.positions[3 * v + 2]);
//        }
//    }

    return models;
}
