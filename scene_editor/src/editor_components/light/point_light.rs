use crate::editor_components::{EditorComponent, ComponentRepresentation};
use roest_runtime::core_components::light::PointLight;
use crate::EditorWorld;

use std::{
    rc::Rc,
    cell::RefCell
};

use legion::prelude::*;

use gtk::prelude::*;
use glib::clone;

impl EditorComponent for PointLight {
    fn component_representation() -> ComponentRepresentation {
        let insert_func = |world: &mut EditorWorld, entity| {
            world.add_component(entity, PointLight::default()).unwrap();
        };

        let ui_func = |world: Rc<RefCell<EditorWorld>>, entity: Entity, paned: &gtk::Paned| {
            let src = include_str!("component_point_light.glade");
            let builder = gtk::Builder::from_string(src);
            let grid: gtk::Frame = builder.get_object("transform_frame").unwrap();

            let constant: gtk::SpinButton = builder.get_object("const").unwrap();
            let linear: gtk::SpinButton = builder.get_object("linear").unwrap();
            let quadratic: gtk::SpinButton = builder.get_object("quad").unwrap();

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
                let point_light = w.world.get_component::<PointLight>(entity).unwrap();

                constant.set_value(point_light.constant as f64);
                linear.set_value(point_light.linear as f64);
                quadratic.set_value(point_light.quadratic as f64);

                ambient_r.set_value(point_light.ambient.x as f64);
                ambient_g.set_value(point_light.ambient.y as f64);
                ambient_b.set_value(point_light.ambient.z as f64);

                diffuse_r.set_value(point_light.diffuse.x as f64);
                diffuse_g.set_value(point_light.diffuse.y as f64);
                diffuse_b.set_value(point_light.diffuse.z as f64);

                specular_r.set_value(point_light.specular.x as f64);
                specular_g.set_value(point_light.specular.y as f64);
                specular_b.set_value(point_light.specular.z as f64);
            }

            constant.connect_value_changed(clone!(@strong world => move |spin_button| {
                let val = spin_button.get_value();
                world
                    .borrow_mut()
                    .world
                    .get_component_mut::<PointLight>(entity)
                    .unwrap()
                    .constant = val as f32;
            }));

            linear.connect_value_changed(clone!(@strong world => move |spin_button| {
                let val = spin_button.get_value();
                world
                    .borrow_mut()
                    .world
                    .get_component_mut::<PointLight>(entity)
                    .unwrap()
                    .linear = val as f32;
            }));

            quadratic.connect_value_changed(clone!(@strong world => move |spin_button| {
                let val = spin_button.get_value();
                world
                    .borrow_mut()
                    .world
                    .get_component_mut::<PointLight>(entity)
                    .unwrap()
                    .quadratic = val as f32;
            }));


            ambient_r.connect_value_changed(clone!(@strong world => move |spin_button| {
                let val = spin_button.get_value();
                world
                    .borrow_mut()
                    .world
                    .get_component_mut::<PointLight>(entity)
                    .unwrap()
                    .ambient.x = val as f32;
            }));
            ambient_g.connect_value_changed(clone!(@strong world => move |spin_button| {
                let val = spin_button.get_value();
                world
                    .borrow_mut()
                    .world
                    .get_component_mut::<PointLight>(entity)
                    .unwrap()
                    .ambient.y = val as f32;
            }));
            ambient_b.connect_value_changed(clone!(@strong world => move |spin_button| {
                let val = spin_button.get_value();
                world
                    .borrow_mut()
                    .world
                    .get_component_mut::<PointLight>(entity)
                    .unwrap()
                    .ambient.z = val as f32;
            }));

            diffuse_r.connect_value_changed(clone!(@strong world => move |spin_button| {
                let val = spin_button.get_value();
                world
                    .borrow_mut()
                    .world
                    .get_component_mut::<PointLight>(entity)
                    .unwrap()
                    .diffuse.x = val as f32;
            }));
            diffuse_g.connect_value_changed(clone!(@strong world => move |spin_button| {
                let val = spin_button.get_value();
                world
                    .borrow_mut()
                    .world
                    .get_component_mut::<PointLight>(entity)
                    .unwrap()
                    .diffuse.y = val as f32;
            }));
            diffuse_b.connect_value_changed(clone!(@strong world => move |spin_button| {
                let val = spin_button.get_value();
                world
                    .borrow_mut()
                    .world
                    .get_component_mut::<PointLight>(entity)
                    .unwrap()
                    .diffuse.z = val as f32;
            }));

            specular_r.connect_value_changed(clone!(@strong world => move |spin_button| {
                let val = spin_button.get_value();
                world
                    .borrow_mut()
                    .world
                    .get_component_mut::<PointLight>(entity)
                    .unwrap()
                    .specular.x = val as f32;
            }));
            specular_g.connect_value_changed(clone!(@strong world => move |spin_button| {
                let val = spin_button.get_value();
                world
                    .borrow_mut()
                    .world
                    .get_component_mut::<PointLight>(entity)
                    .unwrap()
                    .specular.y = val as f32;
            }));
            specular_b.connect_value_changed(clone!(@strong world => move |spin_button| {
                let val = spin_button.get_value();
                world
                    .borrow_mut()
                    .world
                    .get_component_mut::<PointLight>(entity)
                    .unwrap()
                    .specular.z = val as f32;
            }));


            paned.add2(&grid);
            paned.set_child_shrink(&grid, false)
        };

        ComponentRepresentation::new("Point Light".to_string(), insert_func, ui_func)
    }
}