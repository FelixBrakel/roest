use std::path::Path;
use clap::{App, load_yaml};
use gl_renderer::IndexedVertArray;
use gl_renderer::data::vertex_data::ColoredVertex;
use std::fs;
use std::io::Write;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let input = matches.value_of("INPUT").unwrap();
    let output = matches.value_of("OUTPUT").unwrap();
    let obj_file = load_obj_file(input);

    let m = &obj_file[0].mesh;
    let mut verts: Vec<ColoredVertex> = Vec::with_capacity(m.indices.len() / 3);
    println!();

    for v in 0..m.positions.len() / 3 {
        let x = m.positions[3 * v];
        let y = m.positions[3 * v + 1];
        let z = m.positions[3 * v + 2];

        verts.push(ColoredVertex { pos: (x, y, z).into(), clr: (0.5, 0.5, 0.5, 0.5).into() });
    }

    let indexed_vert_array = IndexedVertArray::new(verts, m.indices.clone());
    let serialized = bincode::serialize(&indexed_vert_array).unwrap();

    let mut new_file = fs::File::create(output);
    let mut new_file = new_file.unwrap();

    new_file.write_all(&serialized);

    return ()
}

fn load_obj_file(filepath: &str) -> Vec<tobj::Model> {
    let cornell_box = tobj::load_obj(&Path::new(filepath));
    assert!(cornell_box.is_ok());
    let (models, materials) = cornell_box.unwrap();

    println!("# of models: {}", models.len());
    println!("# of materials: {}", materials.len());
    for (i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;
        println!("model[{}].name = \'{}\'", i, m.name);
        println!("model[{}].mesh.material_id = {:?}", i, mesh.material_id);

        println!("Size of model[{}].indices: {}", i, mesh.indices.len());
        for f in 0..mesh.indices.len() / 3 {
            println!("    idx[{}] = {}, {}, {}.", f, mesh.indices[3 * f],
                     mesh.indices[3 * f + 1], mesh.indices[3 * f + 2]);
        }

        println!("model[{}].vertices: {}", i, mesh.positions.len() / 3);
        assert!(mesh.positions.len() % 3 == 0);
        for v in 0..mesh.positions.len() / 3 {
            println!("    v[{}] = ({}, {}, {})", v, mesh.positions[3 * v],
                     mesh.positions[3 * v + 1], mesh.positions[3 * v + 2]);
        }
    }

    return models;
}
