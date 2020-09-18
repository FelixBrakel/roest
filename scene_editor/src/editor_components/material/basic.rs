use crate::editor_components::{EditorComponent, ComponentRepresentation};
use roest_runtime::core_components::material::Basic;
use crate::EditorWorld;
use std::{
    rc::Rc,
    cell::RefCell
};

use glib::clone;
use gtk::prelude::*;
use legion::prelude::*;

impl EditorComponent for Basic {
    fn component_representation() -> ComponentRepresentation {
        let insert_func = |world: &mut EditorWorld, entity| {
            world.add_component(entity, Basic::default()).unwrap();
        };

        let ui_func = |world: Rc<RefCell<EditorWorld>>, entity: Entity, paned: &gtk::Paned| {
            let src = include_str!("component_basic.glade");
            let builder = gtk::Builder::from_string(src);
            let grid: gtk::Frame = builder.get_object("transform_frame").unwrap();

            let shininess: gtk::SpinButton = builder.get_object("shininess").unwrap();

            let ambient_r: gtk::SpinButton = builder.get_object("ambient_r").unwrap();
            let ambient_g: gtk::SpinButton = builder.get_object("ambient_g").unwrap();
            let ambient_b: gtk::SpinButton = builder.get_object("ambient_b").unwrap();

            let diffuse_r: gtk::SpinButton = builder.get_object("diffuse_r").unwrap();
            let diffuse_g: gtk::SpinButton = builder.get_object("diffuse_g").unwrap();
            let diffuse_b: gtk::SpinButton = builder.get_object("diffuse_b").unwrap();

            let specular_r: gtk::SpinButton = builder.get_object("specular_r").unwrap();
            let specular_g: gtk::SpinButton = builder.get_object("specular_g").unwrap();
            let specular_b: gtk::SpinButton = builder.get_object("specular_b").unwrap();

            {
                let w = world.borrow();
                let basic = w.world.get_component::<Basic>(entity).unwrap();

                shininess.set_value(basic.shininess.d0 as f64);

                ambient_r.set_value(basic.ambient.d0 as f64);
                ambient_g.set_value(basic.ambient.d1 as f64);
                ambient_b.set_value(basic.ambient.d2 as f64);

                diffuse_r.set_value(basic.diffuse.d0 as f64);
                diffuse_g.set_value(basic.diffuse.d1 as f64);
                diffuse_b.set_value(basic.diffuse.d2 as f64);

                specular_r.set_value(basic.specular.d0 as f64);
                specular_g.set_value(basic.specular.d1 as f64);
                specular_b.set_value(basic.specular.d2 as f64);
            }

            shininess.connect_value_changed(clone!(@strong world => move |spin_button| {
                let val = spin_button.get_value();
                world
                    .borrow_mut()
                    .world
                    .get_component_mut::<Basic>(entity)
                    .unwrap()
                    .shininess = (val as f32).into();
            }));

            ambient_r.connect_value_changed(clone!(@strong world => move |spin_button| {
                let val = spin_button.get_value();
                world
                    .borrow_mut()
                    .world
                    .get_component_mut::<Basic>(entity)
                    .unwrap()
                    .ambient.d0 = val as f32;
            }));
            ambient_g.connect_value_changed(clone!(@strong world => move |spin_button| {
                let val = spin_button.get_value();
                world
                    .borrow_mut()
                    .world
                    .get_component_mut::<Basic>(entity)
                    .unwrap()
                    .ambient.d1 = val as f32;
            }));
            ambient_b.connect_value_changed(clone!(@strong world => move |spin_button| {
                let val = spin_button.get_value();
                world
                    .borrow_mut()
                    .world
                    .get_component_mut::<Basic>(entity)
                    .unwrap()
                    .ambient.d2 = val as f32;
            }));

            diffuse_r.connect_value_changed(clone!(@strong world => move |spin_button| {
                let val = spin_button.get_value();
                world
                    .borrow_mut()
                    .world
                    .get_component_mut::<Basic>(entity)
                    .unwrap()
                    .diffuse.d0 = val as f32;
            }));
            diffuse_g.connect_value_changed(clone!(@strong world => move |spin_button| {
                let val = spin_button.get_value();
                world
                    .borrow_mut()
                    .world
                    .get_component_mut::<Basic>(entity)
                    .unwrap()
                    .diffuse.d1 = val as f32;
            }));
            diffuse_b.connect_value_changed(clone!(@strong world => move |spin_button| {
                let val = spin_button.get_value();
                world
                    .borrow_mut()
                    .world
                    .get_component_mut::<Basic>(entity)
                    .unwrap()
                    .diffuse.d2 = val as f32;
            }));

            specular_r.connect_value_changed(clone!(@strong world => move |spin_button| {
                let val = spin_button.get_value();
                world
                    .borrow_mut()
                    .world
                    .get_component_mut::<Basic>(entity)
                    .unwrap()
                    .specular.d0 = val as f32;
            }));
            specular_g.connect_value_changed(clone!(@strong world => move |spin_button| {
                let val = spin_button.get_value();
                world
                    .borrow_mut()
                    .world
                    .get_component_mut::<Basic>(entity)
                    .unwrap()
                    .specular.d1 = val as f32;
            }));
            specular_b.connect_value_changed(clone!(@strong world => move |spin_button| {
                let val = spin_button.get_value();
                world
                    .borrow_mut()
                    .world
                    .get_component_mut::<Basic>(entity)
                    .unwrap()
                    .specular.d2 = val as f32;
            }));


            paned.add2(&grid);
            paned.set_child_shrink(&grid, false)
        };

        ComponentRepresentation::new("Material Basic".to_string(), insert_func, ui_func)
    }
}