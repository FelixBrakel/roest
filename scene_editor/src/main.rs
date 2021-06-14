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
        gpu_blocks::{
            Lights,
            Matrices
        }
    },
    core_components,
    core_systems::{
        resource_manager::data_loaders::ProgramLoader
    },
    create_gl_window
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

use glib::{subclass::prelude::*, clone, GBoxed};
use gtk::{TreeStore, TreeIter, TreeView, Builder, FrameClass};
use gdk::FrameClock;

use gl_renderer::{
    Viewport,
    ColorBuffer,
    uniform_buffer::InterfaceBlock
};

use shared_library::dynamic_library::DynamicLibrary;
use roest_runtime::core_systems::resource_manager::Loader;

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
            components: Vec::new()
        };

        let root_entity = world.insert(
            (Root,),
            iter::once(
                (comp_vis,)
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
        if !self.world.has_component::<T>(entity) {
            let mut vis_comp = self.world.get_component_mut::<ComponentVisualization>(entity).unwrap();
            vis_comp.components.push(T::component_representation());
        }
        self.world.add_component(entity, component)
    }

    pub fn remove_component<T>(&mut self, entity: Entity) -> Result<(), EntityMutationError>
    where
        T: EditorComponent
    {
        //TODO: make this not an array but a hashmap for easy removal.
        //let mut vis_comp = self.world.get_component_mut::<ComponentVisualization>(entity).unwrap();
        //let repr = T::component_representation();
        //vis_comp.components.remove();
        self.world.remove_component::<T>(entity)
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

#[derive(Clone, Debug, PartialEq, Eq, GBoxed)]
#[gboxed(type_name = "TreeKeyWrapper")]
struct TreeKeyWrapper(Box<TreeKey>);

fn main() -> anyhow::Result<()> {
    let universe = Universe::new();
    let editor_world = Rc::new(RefCell::new(EditorWorld::new(universe.create_world())));
    let resources = Rc::new(RefCell::new(Resources::default()));
    let render_schedule = Rc::new(RefCell::new(Schedule::builder().add_thread_local().build()));

    let root = editor_world.borrow().scene_graph.tree().root();
    let color_buffer = Rc::new(ColorBuffer::from_color(na::Vector3::new(0.3, 0.3, 0.5)));

    let builder = build_ui(editor_world.clone(), root)?;

    let window: gtk::Window = builder.get_object("window_main").unwrap();
    create_gl_window();
    let program = ProgramLoader::new().load("resources/shaders/basic").unwrap();
    let lights_block = InterfaceBlock::<Lights>::new(&program, "Lights", 1);
    let matrices_block = InterfaceBlock::<Matrices>::new(&program, "Matrices", 2);
    let material_block = InterfaceBlock::<core_components::material::Basic>::new(&program, "Material", 3);

    resources.borrow_mut().insert(program);
    resources.borrow_mut().insert(lights_block);
    resources.borrow_mut().insert(matrices_block);
    resources.borrow_mut().insert(material_block);

    // let gl_area: gtk::GLArea = builder.get_object("gl_area").unwrap();

    // epoxy::load_with(|s| {
    //     unsafe {
    //         match DynamicLibrary::open(None).unwrap().symbol(s) {
    //             Ok(v) => v,
    //             Err(_) => std::ptr::null(),
    //         }
    //     }
    // });

    // gl::load_with(epoxy::get_proc_addr);
    // gl_area.set_required_version(4, 5);

    // gl_area.connect_realize(clone!(@strong resources, @strong color_buffer => move |area| {
    //     area.make_current();
    //     area.set_auto_render(false);
    //
    //     let (maj, min) = area.get_required_version();
    //     println!("OPENGL VERSION: {}.{}", maj, min);
    //     area.set_has_depth_buffer(true);
    //
    //     unsafe {
            // gl::Enable(gl::DEPTH_TEST);
            // gl::Enable(gl::DEBUG_OUTPUT);
            //
            // gl::DebugMessageCallback(Some(gl::error_callback), std::ptr::null())
        // }

        // let width = gl_area.get_allocated_width();
        // let height = gl_area.get_allocated_height();
        // let viewport = Rc::new(RefCell::new(Viewport::for_window(width, height)));
        // viewport.borrow().set_used();

        // color_buffer.set_used();

        // let program = ProgramLoader::new().load("resources/shaders/basic").unwrap();
        // let lights_block = InterfaceBlock::<Lights>::new(&program, "Lights", 1);
        // let matrices_block = InterfaceBlock::<Matrices>::new(&program, "Matrices", 2);
        // let material_block = InterfaceBlock::<core_components::material::Basic>::new(&program, "Material", 3);
        //
        // resources.borrow_mut().insert(program);
        // resources.borrow_mut().insert(lights_block);
        // resources.borrow_mut().insert(matrices_block);
        // resources.borrow_mut().insert(material_block);
    // }));

    // gl_area.connect_resize(clone!(@strong viewport => move |_, w, h| {
    //     viewport.borrow_mut().update_size(w, h);
    //     viewport.borrow().set_used();
    // }));

    // gl_area.connect_render(clone!(@strong editor_world, @strong resources, @strong render_schedule, @strong resources, @strong color_buffer => move |area, _| {
    //     color_buffer.clear();
    //     render_schedule.borrow_mut().execute(&mut editor_world.borrow_mut().world, &mut resources.borrow_mut());
    //     Inhibit(false)
    // }));

    //NOTE: this is temporary until I figure out how to queue renders from an update loop.
    // gl_area.add_tick_callback(move |area, _| {
    //     area.queue_render();
    //     Continue(true)
    // });

    window.show_all();
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
    let button_remove_component: gtk::Button = builder.get_object("button_remove_component").unwrap();
    let cell_renderer: gtk::CellRendererText = builder.get_object("cell_renderer_entity_name").unwrap();
    let paned: gtk::Paned = builder.get_object("paned2").unwrap();
    let component_view: TreeView = builder.get_object("component_view").unwrap();
    let component_selection = component_view.get_selection();

    {
        let src = include_str!("editor_components/component_none.glade");
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
      editor_world.borrow_mut().add_entity(root, format!("Dummy Entity {}", i))?;
    }

    let root_key = Rc::new(root);

    editor_world.borrow().insert_into_tree_store(&store);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let global_component_listbox: gtk::ListBox = builder.get_object("component_listbox").unwrap();

    let mut component_registry = Vec::new();
    let mut component_buttons = Vec::new();
    let mut component_listboxrows = Vec::new();
    component_registry.push(core_components::Transform::component_representation());
    component_registry.push(core_components::Camera::component_representation());
    component_registry.push(core_components::light::PointLight::component_representation());
    component_registry.push(core_components::IndexedMesh::component_representation());
    component_registry.push(core_components::material::Basic::component_representation());

    let gl_area: gtk::GLArea = builder.get_object("gl_area").unwrap();

    for component in component_registry {
        let button = gtk::Button::with_label(&component.name());
        button.connect_clicked(clone!(@strong editor_world, @strong tree_selection, @strong component_list, @strong gl_area => move |_| {
            gl_area.make_current();
            let s = tree_selection.get_selected();
            let key = match &s {
                Some((model, iter)) => {
                    let v = model.get_value(&iter, 1);
                    let k = v.get::<&TreeKeyWrapper>().unwrap().unwrap();
                    *k.0
                },
                None => return
            };

            let entity = editor_world.borrow().scene_graph.tree().value(key).unwrap().clone();
            let func = component.insert_func();
            (func)(&mut editor_world.borrow_mut(), entity);

            component_list.clear();
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

        component_buttons.push(button);
    }

    for button in component_buttons.iter() {
        let row = gtk::ListBoxRow::new();
        row.add(button);

        component_listboxrows.push(row);
    }

    for row in component_listboxrows.iter() {
        global_component_listbox.add(row);
    }

    global_component_listbox.show_all();

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
                let src = include_str!("editor_components/component_none.glade");
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
                let src = include_str!("editor_components/component_none.glade");
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

    Ok(builder)
}
