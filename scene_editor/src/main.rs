mod editor_components;

use std::{
    iter,
    rc::Rc,
    cell::RefCell
};

use nalgebra as na;

use roest_runtime::{
    core_resources::{
        scene_graph::SceneGraph,
    },
    core_components,
    core_systems::{
        renderer::RendererSystem,
    }
};

use legion::{
    prelude::*,
    world::EntityMutationError,
};
use slotted_tree::{TreeKey, TreeError, SecondaryTree};

use editor_components::{
    EditorComponent,
    ComponentRepresentation
};

use gtk::prelude::*;

use glib::{
    subclass::prelude::*,
    clone,
    GBoxed
};
use gtk::{TreeStore, TreeIter, TreeView, Builder};

use gl_renderer::{
    Viewport,
    ColorBuffer,
};

use shared_library::dynamic_library::DynamicLibrary;
use roest_runtime::core_components::Transform;

pub struct ComponentVisualization {
    components: Vec<ComponentRepresentation>
}

pub struct EditorWorld {
    world: World,
    scene_graph: SceneGraph,
    names: SecondaryTree<String>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Root;

impl EditorWorld {
    pub fn new(mut world: World) -> Self {

        let comp_vis = ComponentVisualization {
            components: vec![Empty.as_component_representation()]
        };

        let root_entity = world.insert(
            (Root,),
            iter::once(
                (Empty, comp_vis)
            )
        )[0];

        let scene_graph = SceneGraph::new(root_entity).unwrap();
        let names = scene_graph.tree().map_to_secondary(|_,_| String::from("Root"));
        EditorWorld {
            world,
            scene_graph,
            names,
        }
    }

    pub fn add_entity<S: Into<String>>(&mut self, parent: TreeKey, name: S) -> Result<TreeKey, TreeError> {
        let comp_vis = ComponentVisualization {
            components: Vec::new()
        };
        let entity = self.world.insert(
            (),
            iter::once(
                (comp_vis,)
            )
        )[0];

        let key = self.scene_graph.add_child(parent, entity)?;
        self.names.insert(key, name.into());
        Ok(key)
    }

    pub fn remove_entity(&mut self, entity: TreeKey) {
        let entities = self.scene_graph.tree_mut().remove_tree(entity);
        for e in entities {
            match e {
                Some(val) => { self.world.delete(val); },
                None => println!("Old TreeKey in children, skipping deletion")
            }
        }
    }

    pub fn add_component<T>(&mut self, entity: Entity, component: T) -> Result<(), EntityMutationError>
    where
        T: EditorComponent
    {
        {
            let mut vis_comp = self.world.get_component_mut::<ComponentVisualization>(entity).unwrap();

            vis_comp.components.push(component.as_component_representation());
        }
        self.world.add_component(entity, component)
    }

    pub fn insert_into_tree_store(&self, store: &TreeStore) {
        let tree = self.scene_graph.tree();
        let root = self.scene_graph.tree().root();

        let children = match tree.children(root) {
            Some(ch) => ch,
            None => return
        };

        for ch in children {
            self.insert_into_tree_store_dfs(None, *ch, store);
        }
    }

    fn insert_into_tree_store_dfs(&self, parent: Option<&TreeIter>, curr_node: TreeKey, store: &TreeStore) {
        let tree = self.scene_graph.tree();
        let name = self.names.get(curr_node).unwrap();

        let iter = store.insert_with_values(
            parent,
            None,
            &[0, 1],
            &[name, &TreeKeyWrapper(Box::new(curr_node))]
        );


        let children = match tree.children(curr_node) {
            Some(ch) => ch,
            None => return
        };

        for ch in children {
            self.insert_into_tree_store_dfs(Some(&iter), *ch, store);
        }
    }
}

pub struct Empty;

impl EditorComponent for Empty {
    fn as_component_representation(&self) -> ComponentRepresentation {
        let insert_func = |world: &mut World, entity: Entity| {
            world.add_component(entity, Empty).unwrap();
        };

        let ui_func = |_: Rc<RefCell<EditorWorld>>, _: Entity, _: &gtk::Paned| {};

        ComponentRepresentation::new("Empty".to_string(), insert_func, ui_func)
    }
}

pub struct ChildComponent(u32);

impl ChildComponent {
    fn attach_ui(&self, paned: &gtk::Paned) {
        let src = include_str!("editor_components/component_transform.glade");
        let builder = gtk::Builder::from_string(src);

        let grid: gtk::Frame = builder.get_object("transform_frame").unwrap();

        paned.add2(&grid);
        paned.set_child_shrink(&grid, false)

        // connect signals, set values.
    }
}

impl EditorComponent for ChildComponent {
    fn as_component_representation(&self) -> ComponentRepresentation {
        let insert_func = |world: &mut World, entity| {
            world.add_component(entity, ChildComponent(0)).unwrap();
        };

        let ui_func = |world: Rc<RefCell<EditorWorld>>, entity: Entity, paned: &gtk::Paned| {
            world.borrow().world.get_component::<ChildComponent>(entity).unwrap().attach_ui(paned);
        };

        ComponentRepresentation::new(format!("Child {}", self.0), insert_func, ui_func)
    }
}

// impl EditorComponent for Transform {
//     fn as_component_representation(&self) -> ComponentRepresentation {
//         let insert_func = |world: &mut World, entity| {
//             world.add_component(entity, Transform::from_defaults()).unwrap();
//         };
//
//         let ui_func = |world: Rc<RefCell<EditorWorld>>, entity: Entity, paned: &gtk::Paned| {
//             let src = include_str!("component_transform.glade");
//             let builder = gtk::Builder::from_string(src);
//
//             let grid: gtk::Frame = builder.get_object("transform_frame").unwrap();
//
//             let pos_x: gtk::SpinButton = builder.get_object("pos_x").unwrap();
//             let pos_y: gtk::SpinButton = builder.get_object("pos_y").unwrap();
//             let pos_z: gtk::SpinButton = builder.get_object("pos_z").unwrap();
//
//             let rot_x: gtk::SpinButton = builder.get_object("rot_x").unwrap();
//             let rot_y: gtk::SpinButton = builder.get_object("rot_y").unwrap();
//             let rot_z: gtk::SpinButton = builder.get_object("rot_z").unwrap();
//
//             let scale_x: gtk::SpinButton = builder.get_object("scale_x").unwrap();
//             let scale_y: gtk::SpinButton = builder.get_object("scale_y").unwrap();
//             let scale_z: gtk::SpinButton = builder.get_object("scale_z").unwrap();
//
//             {
//                 let w = world.borrow();
//                 let component = w.world.get_component::<Transform>(entity).unwrap();
//                 let pos = component.translation();
//
//                 pos_x.set_value(pos.x as f64);
//                 pos_y.set_value(pos.y as f64);
//                 pos_z.set_value(pos.z as f64);
//
//                 let rot = component.rotation().euler_angles();
//
//                 rot_x.set_value(rot.0 as f64);
//                 rot_y.set_value(rot.1 as f64);
//                 rot_z.set_value(rot.2 as f64);
//
//                 let scale = component.scale();
//
//                 scale_x.set_value(scale.x as f64);
//                 scale_y.set_value(scale.y as f64);
//                 scale_z.set_value(scale.z as f64);
//             }
//
//             pos_x.connect_value_changed(clone!(@strong world => move |spin_button| {
//                 let val = spin_button.get_value();
//                 world
//                     .borrow_mut()
//                     .world
//                     .get_component_mut::<Transform>(entity)
//                     .unwrap()
//                     .translate_x(val as f32);
//             }));
//             pos_y.connect_value_changed(clone!(@strong world => move |spin_button| {
//                 let val = spin_button.get_value();
//                 world
//                     .borrow_mut()
//                     .world
//                     .get_component_mut::<Transform>(entity)
//                     .unwrap()
//                     .translate_y(val as f32);
//             }));
//             pos_z.connect_value_changed(clone!(@strong world => move |spin_button| {
//                 let val = spin_button.get_value();
//                 world
//                     .borrow_mut()
//                     .world
//                     .get_component_mut::<Transform>(entity)
//                     .unwrap()
//                     .translate_z(val as f32);
//             }));
//
//             rot_x.connect_value_changed(clone!(@strong world => move |spin_button| {
//                 let val = spin_button.get_value();
//                 world
//                     .borrow_mut()
//                     .world
//                     .get_component_mut::<Transform>(entity)
//                     .unwrap()
//                     .rotation_x(val as f32);
//             }));
//             rot_y.connect_value_changed(clone!(@strong world => move |spin_button| {
//                 let val = spin_button.get_value();
//                 world
//                     .borrow_mut()
//                     .world
//                     .get_component_mut::<Transform>(entity)
//                     .unwrap()
//                     .rotation_y(val as f32);
//             }));
//             rot_z.connect_value_changed(clone!(@strong world => move |spin_button| {
//                 let val = spin_button.get_value();
//                 world
//                     .borrow_mut()
//                     .world
//                     .get_component_mut::<Transform>(entity)
//                     .unwrap()
//                     .rotation_z(val as f32);
//             }));
//
//             scale_x.connect_value_changed(clone!(@strong world => move |spin_button| {
//                 let val = spin_button.get_value();
//                 world
//                     .borrow_mut()
//                     .world
//                     .get_component_mut::<Transform>(entity)
//                     .unwrap()
//                     .scale_x(val as f32);
//             }));
//             scale_y.connect_value_changed(clone!(@strong world => move |spin_button| {
//                 let val = spin_button.get_value();
//                 world
//                     .borrow_mut()
//                     .world
//                     .get_component_mut::<Transform>(entity)
//                     .unwrap()
//                     .scale_y(val as f32);
//             }));
//             scale_z.connect_value_changed(clone!(@strong world => move |spin_button| {
//                 let val = spin_button.get_value();
//                 world
//                     .borrow_mut()
//                     .world
//                     .get_component_mut::<Transform>(entity)
//                     .unwrap()
//                     .scale_z(val as f32);
//             }));
//
//             paned.add2(&grid);
//             paned.set_child_shrink(&grid, false)
//         };
//
//
//         ComponentRepresentation::new("Transform".to_string(), insert_func, ui_func)
//     }
// }

#[derive(Clone, Copy, Debug, PartialEq)]
struct Child(u32);

#[derive(Clone, Debug, PartialEq, Eq, GBoxed)]
#[gboxed(type_name = "TreeKeyWrapper")]
struct TreeKeyWrapper(Box<TreeKey>);


fn main() -> anyhow::Result<()> {
    let universe = Universe::new();
    let editor_world = Rc::new(RefCell::new(EditorWorld::new(universe.create_world())));
    let root = editor_world.borrow().scene_graph.tree().root();
    let resources = Rc::new(RefCell::new(Resources::default()));

    let mut component_registry = Vec::new();
    let tmp = core_components::Transform::from_defaults();
    component_registry.push(tmp.as_component_representation());

    let builder = build_ui(editor_world.clone(), root)?;

    let gl_area: gtk::GLArea = builder.get_object("gl_area").unwrap();

    gl_area.connect_realize(|area| {
        area.set_required_version(4, 5)
    });

    gl_area.make_current();
    epoxy::load_with(|s| {
        unsafe {
            match DynamicLibrary::open(None).unwrap().symbol(s) {
                Ok(v) => v,
                Err(_) => std::ptr::null(),
            }
        }
    });

    gl::load_with(epoxy::get_proc_addr);

    let width = gl_area.get_allocated_width();
    let height = gl_area.get_allocated_height();
    let viewport = Rc::new(RefCell::new(Viewport::for_window(width, height)));
    viewport.borrow().set_used();

    let color_buffer = ColorBuffer::from_color(na::Vector3::new(0.3, 0.3, 0.5));
    color_buffer.set_used();

    gl_area.connect_realize(|_| {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::DEBUG_OUTPUT);

            gl::DebugMessageCallback(Some(gl::error_callback), std::ptr::null())
        }
    });

    gl_area.connect_resize(clone!(@strong viewport => move |_, w, h| {
        viewport.borrow_mut().update_size(w, h);
        viewport.borrow().set_used();
    }));

    let renderer = RendererSystem::system();
    let schedule = Rc::new(RefCell::new(Schedule::builder().add_thread_local(renderer).build()));

    gl_area.connect_render(clone!(@strong editor_world, @strong resources, @strong schedule => move |_, _| {
        color_buffer.clear();
        // schedule.borrow_mut().execute(&mut editor_world.borrow_mut().world, &mut resources.borrow_mut());
        Inhibit(false)
    }));

    gtk::main();

    Ok(())
}

fn build_ui(editor_world: Rc<RefCell<EditorWorld>>, root: TreeKey) -> anyhow::Result<Builder> {
    gtk::init().expect("Failed to initialize GTK.");

    let glade_src = include_str!("scene_editor.glade");

    let builder = gtk::Builder::from_string(glade_src);

    let window: gtk::Window = builder.get_object("window_main").unwrap();
    let tree: gtk::TreeView = builder.get_object("scene_graph").unwrap();
    let button_add: gtk::Button = builder.get_object("button_add").unwrap();
    let button_remove: gtk::Button = builder.get_object("button_remove").unwrap();
    let cell_renderer: gtk::CellRendererText = builder.get_object("cell_renderer_entity_name").unwrap();
    let paned: gtk::Paned = builder.get_object("paned2").unwrap();
    let component_view: TreeView = builder.get_object("component_view").unwrap();
    let component_selection = component_view.get_selection();

    {
        let src = include_str!("component_none.glade");
        let build = gtk::Builder::from_string(src);
        let frame: gtk::Frame = build.get_object("empty_frame").unwrap();
        paned.add2(&frame);
    }

    let component_list: gtk::ListStore = builder.get_object("component_liststore").unwrap();

    let tys = &[String::static_type(), TreeKeyWrapper::get_type()];
    let store = TreeStore::new(tys);
    tree.set_model(Some(&store));

    let tree_selection = tree.get_selection();
    let model = tree.get_model().unwrap();

    for i in 0..4 {
        let handle = editor_world.borrow_mut().add_entity(root, format!("Child {}", i))?;
        let entity = editor_world.borrow().scene_graph.tree().value(handle).unwrap().clone();
        editor_world.borrow_mut().add_component(entity, Empty)?;
        editor_world.borrow_mut().add_component(entity, Transform::from_defaults())?;
    }

    let root_key = Rc::new(root);

    editor_world.borrow().insert_into_tree_store(&store);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    button_add.connect_clicked(clone!(@strong editor_world, @strong store, @strong tree_selection, @strong root_key => move |_| {
        let s = tree_selection.get_selected();
        let (key, iter) = match &s {
            Some((model, iter)) => {
                let v = model.get_value(&iter, 1);
                let k = v.get::<&TreeKeyWrapper>().unwrap().unwrap();
                (*k.0, Some(iter))
            },
            None => (*root_key, None)
        };

        let handle = editor_world.borrow_mut().add_entity(key, "Entity").unwrap();
        store.insert_with_values(
            iter,
            None,
            &[0, 1],
            &[&"Entity", &TreeKeyWrapper(Box::new(handle))]
        );
    }));

    button_remove.connect_clicked(clone!(@strong editor_world, @strong store, @strong tree_selection, @strong root_key => move |_| {
        let s = tree_selection.get_selected();
        let (key, iter) = match &s {
            Some((model, iter)) => {
                let v = model.get_value(&iter, 1);
                let k = v.get::<&TreeKeyWrapper>().unwrap().unwrap();
                (*k.0, iter)
            },
            None => return
        };

        store.remove(iter);
        editor_world.borrow_mut().remove_entity(key);
    }));

    cell_renderer.connect_edited(clone!(@weak model, @strong store => move |_, path, name| {
        let iter = model.get_iter(&path);
        store.set_value(&iter.unwrap(), 0, &name.to_value())
    }));

    tree_selection.connect_changed(clone!(@strong editor_world, @strong component_list => move |selection| {
        component_list.clear();
        let s = selection.get_selected();
        let key = match &s {
            Some((model, iter)) => {
                let v = model.get_value(&iter, 1);
                let k = v.get::<&TreeKeyWrapper>().unwrap().unwrap();
                *k.0
            },
            None => return
        };

        let entity = editor_world.borrow().scene_graph.tree().value(key).unwrap().clone();

        let e_world = editor_world.borrow();
        let components = e_world.world.get_component::<ComponentVisualization>(entity).unwrap();

        for (i, comp) in components.components.iter().enumerate() {
            component_list.insert_with_values(
                None,
                &[0, 1],
                &[&comp.name(), &(i as u64)]
            );
        }
    }));

    component_selection.connect_changed(clone!(@strong editor_world, @strong tree_selection, @strong paned => move |selection| {
        let comp_select = selection.get_selected();
        match paned.get_child2() {
            Some(child) => paned.remove(&child),
            None => ()
        };

        let idx = match &comp_select {
            Some((model, iter)) => {
                let v = model.get_value(&iter, 1);
                let i = v.get::<u64>().unwrap().unwrap();
                i
            },
            None => {
                let src = include_str!("component_none.glade");
                let builder = gtk::Builder::from_string(src);
                let frame: gtk::Frame = builder.get_object("empty_frame").unwrap();
                paned.add2(&frame);

                return
            }
        };

        let s = tree_selection.get_selected();
        let key= match &s {
            Some((model, iter)) => {
                let v = model.get_value(&iter, 1);
                let k = v.get::<&TreeKeyWrapper>().unwrap().unwrap();
                *k.0
            },
            None => {
                let src = include_str!("component_none.glade");
                let builder = gtk::Builder::from_string(src);
                let frame: gtk::Frame = builder.get_object("empty_frame").unwrap();
                paned.add2(&frame);

                return
            }
        };

        let entity = editor_world.borrow().scene_graph.tree().value(key).unwrap().clone();

        if idx as usize >= editor_world
            .borrow()
            .world
            .get_component::<ComponentVisualization>(entity)
            .unwrap()
            .components
            .len() {
            selection.unselect_all();
            return
        }

        let func = editor_world
            .borrow()
            .world
            .get_component::<ComponentVisualization>(entity)
            .unwrap()
            .components[idx as usize]
            .ui_func();

        (func)(editor_world.clone(), entity, &paned);
    }));

    window.show_all();
    Ok(builder)
}
