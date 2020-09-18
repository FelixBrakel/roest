use roest_runtime::core_components::Transform;
use super::{
    EditorComponent,
    ComponentRepresentation
};

use gtk::prelude::*;
use glib::clone;

use std::{
    rc::Rc,
    cell::RefCell
};

use crate::EditorWorld;
use legion::prelude::*;

impl EditorComponent for Transform {
    fn component_representation() -> ComponentRepresentation {
        let insert_func = |world: &mut EditorWorld, entity| {
            world.add_component(entity, Transform::default()).unwrap();
        };

        let ui_func = |world: Rc<RefCell<EditorWorld>>, entity: Entity, paned: &gtk::Paned| {
            let src = include_str!("component_transform.glade");
            let builder = gtk::Builder::from_string(src);

            let grid: gtk::Frame = builder.get_object("transform_frame").unwrap();

            let pos_x: gtk::SpinButton = builder.get_object("pos_x").unwrap();
            let pos_y: gtk::SpinButton = builder.get_object("pos_y").unwrap();
            let pos_z: gtk::SpinButton = builder.get_object("pos_z").unwrap();

            let rot_x: gtk::SpinButton = builder.get_object("rot_x").unwrap();
            let rot_y: gtk::SpinButton = builder.get_object("rot_y").unwrap();
            let rot_z: gtk::SpinButton = builder.get_object("rot_z").unwrap();

            let scale_x: gtk::SpinButton = builder.get_object("scale_x").unwrap();
            let scale_y: gtk::SpinButton = builder.get_object("scale_y").unwrap();
            let scale_z: gtk::SpinButton = builder.get_object("scale_z").unwrap();

            {
                let w = world.borrow();
                let component = w.world.get_component::<Transform>(entity).unwrap();
                let pos = component.translation();

                pos_x.set_value(pos.x as f64);
                pos_y.set_value(pos.y as f64);
                pos_z.set_value(pos.z as f64);

                let rot = component.rotation().euler_angles();

                // We display in degrees so we need to convert. This needs to happen the other way around when setting
                // the values.
                rot_x.set_value((rot.0 * 57.295779513) as f64);
                rot_y.set_value((rot.1 * 57.295779513) as f64);
                rot_z.set_value((rot.2 * 57.295779513) as f64);

                let scale = component.scale();

                scale_x.set_value(scale.x as f64);
                scale_y.set_value(scale.y as f64);
                scale_z.set_value(scale.z as f64);
            }

            pos_x.connect_value_changed(clone!(@strong world => move |spin_button| {
                let val = spin_button.get_value();
                world
                    .borrow_mut()
                    .world
                    .get_component_mut::<Transform>(entity)
                    .unwrap()
                    .position_x(val as f32);
            }));
            pos_y.connect_value_changed(clone!(@strong world => move |spin_button| {
                let val = spin_button.get_value();
                world
                    .borrow_mut()
                    .world
                    .get_component_mut::<Transform>(entity)
                    .unwrap()
                    .position_y(val as f32);
            }));
            pos_z.connect_value_changed(clone!(@strong world => move |spin_button| {
                let val = spin_button.get_value();
                world
                    .borrow_mut()
                    .world
                    .get_component_mut::<Transform>(entity)
                    .unwrap()
                    .position_z(val as f32);
            }));

            rot_x.connect_value_changed(clone!(@strong world, @strong rot_y, @strong rot_z => move |spin_button| {
                let x = spin_button.get_value();
                let y = rot_y.get_value();
                let z = rot_z.get_value();

                world
                    .borrow_mut()
                    .world
                    .get_component_mut::<Transform>(entity)
                    .unwrap()
                    .set_rotation(
                        x as f32 * 0.01745329252_f32,
                        y as f32 * 0.01745329252_f32,
                        z as f32 * 0.01745329252_f32
                    );
            }));
            rot_y.connect_value_changed(clone!(@strong world, @strong rot_x, @strong rot_z => move |spin_button| {
                let x = rot_x.get_value();
                let y = spin_button.get_value();
                let z = rot_z.get_value();
                world
                    .borrow_mut()
                    .world
                    .get_component_mut::<Transform>(entity)
                    .unwrap()
                    .set_rotation(
                        x as f32 * 0.01745329252_f32,
                        y as f32 * 0.01745329252_f32,
                        z as f32 * 0.01745329252_f32
                    );
            }));
            rot_z.connect_value_changed(clone!(@strong world, @strong rot_x, @strong rot_y => move |spin_button| {
                let x = rot_x.get_value();
                let y = rot_y.get_value();
                let z = spin_button.get_value();
                world
                    .borrow_mut()
                    .world
                    .get_component_mut::<Transform>(entity)
                    .unwrap()
                    .set_rotation(
                        x as f32 * 0.01745329252_f32,
                        y as f32 * 0.01745329252_f32,
                        z as f32 * 0.01745329252_f32
                    );
            }));

            scale_x.connect_value_changed(clone!(@strong world => move |spin_button| {
                let val = spin_button.get_value();
                world
                    .borrow_mut()
                    .world
                    .get_component_mut::<Transform>(entity)
                    .unwrap()
                    .scale_x(val as f32);
            }));
            scale_y.connect_value_changed(clone!(@strong world => move |spin_button| {
                let val = spin_button.get_value();
                world
                    .borrow_mut()
                    .world
                    .get_component_mut::<Transform>(entity)
                    .unwrap()
                    .scale_y(val as f32);
            }));
            scale_z.connect_value_changed(clone!(@strong world => move |spin_button| {
                let val = spin_button.get_value();
                world
                    .borrow_mut()
                    .world
                    .get_component_mut::<Transform>(entity)
                    .unwrap()
                    .scale_z(val as f32);
            }));

            paned.add2(&grid);
            paned.set_child_shrink(&grid, false)
        };


        ComponentRepresentation::new("Transform".to_string(), insert_func, ui_func)
    }
}