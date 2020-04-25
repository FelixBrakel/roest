extern crate gl_generator;

use gl_generator::{Registry, Fallbacks, Api, Profile, GlobalGenerator};
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut file_gl = File::create(&Path::new(&out_dir).join("bindings.rs")).unwrap();
    let registry = Registry::new(Api::Gl, (4, 5), Profile::Core, Fallbacks::All, []);

    registry.write_bindings(GlobalGenerator, &mut file_gl).unwrap();
}