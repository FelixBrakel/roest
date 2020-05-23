use std::path::Path;
use clap::{App, load_yaml};
use gl_renderer::{IndexedVertArray, VertexAttribPointers};
use gl_renderer::vertex::{NormalVertex, BasicVertex};
use std::fs;
use std::io::Write;
use std::f32;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let input = matches.value_of("INPUT").unwrap();
    let output = matches.value_of("OUTPUT").unwrap();
    let obj_file = load_obj_file(input);

    let m = &obj_file[0].mesh;

    let mut tuples: Vec<(f32, f32, f32)> = Vec::with_capacity(m.indices.len() / 3);
    let mut verts: Vec<BasicVertex> = Vec::with_capacity(m.indices.len() / 3);

    let mut max: f32 = 0.;
    let mut min: f32 = 0.;

    let mut avg_x = 0.;
    let mut avg_y = 0.;
    let mut avg_z = 0.;

    for v in 0..m.positions.len() / 3 {
        let x = m.positions[3 * v];
        let y = m.positions[3 * v + 1];
        let z = m.positions[3 * v + 2];

        avg_x += m.positions[3 * v];
        avg_y += m.positions[3 * v + 1];
        avg_z += m.positions[3 * v + 2];

        tuples.push((x, y, z));
    }

    avg_x /= (m.positions.len() / 3) as f32;
    avg_y /= (m.positions.len() / 3) as f32;
    avg_z /= (m.positions.len() / 3) as f32;

    for (x, y, z) in tuples.iter_mut() {
        *x -= avg_x;
        *y -= avg_y;
        *z -= avg_z;

        max = max.max(*x).max(*y).max(*z);
        min = min.min(*x).min(*y).min(*z);
    }

    for (i, (x, y, z)) in tuples.iter().enumerate() {
        let x_normalized = (x - min) / (max - min) * 2. - 1.;
        let y_normalized = (y - min) / (max - min) * 2. - 1.;
        let z_normalized = (z - min) / (max - min) * 2. - 1.;

        let x_n = m.normals[i*3];
        let y_n = m.normals[i*3 + 1];
        let z_n = m.normals[i*3 + 2];

        let u = m.texcoords[i*3];
        let v = m.texcoords[i*3 + 1];

        verts.push(BasicVertex {
            pos: (x_normalized, y_normalized, z_normalized).into(),
            normal: (x_n, y_n, z_n).into(),
            uv: (u, v).into()
        });
    }

    let indexed_vert_array = IndexedVertArray::new(verts, m.indices.clone());
    let serialized = bincode::serialize(&indexed_vert_array).unwrap();

    let mut new_file = fs::File::create(output).unwrap();

    new_file.write_all(&serialized).unwrap();
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
