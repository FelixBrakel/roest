mod editor_component;

use tui::{
    Terminal,
    backend::TermionBackend,
    widgets::{Block, Borders},
    layout::{Layout, Constraint, Direction},
    style::{Style, Color, Modifier},
};

use termion::{
    raw::IntoRawMode,
    event::Key,
    input::{MouseTerminal},
    screen::AlternateScreen
};

use std::{
    io,
    iter,
};

use roest_runtime::{
    core_resources::scene_graph::SceneGraph
};

use legion::{
    prelude::*,
    world::EntityMutationError,
    storage::Tag
};

use slotted_tree::{
    TreeKey,
    TreeError
};
use crate::editor_component::{EditorComponent, ComponentRepresentation};
use crate::tree_list::{StatefulTreeList};
use tui::widgets::{List, Text};

use gtk::prelude::*;
use gio::prelude::*;

use gtk::{Application, ApplicationWindow, Button, Builder, TreeModel, TreeStore};

mod event;
mod tree_list;

use event::{Events, Event};

pub struct ComponentVisualization {
    components: Vec<ComponentRepresentation>
}

pub struct EditorWorld {
    world: World,
    scene_graph: SceneGraph,
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

        EditorWorld {
            world,
            scene_graph: SceneGraph::new(root_entity).unwrap()
        }
    }

    pub fn add_entity<T: Tag>(&mut self, parent: TreeKey, tag: T) -> Result<TreeKey, TreeError> {
        let comp_vis = ComponentVisualization {
            components: Vec::new()
        };
        let entity = self.world.insert(
            (tag,),
            iter::once(
                (comp_vis,)
            )
        )[0];

        self.scene_graph.add_child(parent, entity)
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
}

pub struct Empty;

impl EditorComponent for Empty {
    fn as_component_representation(&self) -> ComponentRepresentation {
        ComponentRepresentation::new("Empty".to_string())
    }
}

pub struct ChildComponent(u32);

impl EditorComponent for ChildComponent {
    fn as_component_representation(&self) -> ComponentRepresentation {
        ComponentRepresentation::new(format!("Child {}", self.0))
    }
}



#[derive(Clone, Copy, Debug, PartialEq)]
struct Child(u32);

fn main() -> anyhow::Result<()> {
    gtk::init().expect("Failed to initialize GTK.");

    let glade_src = include_str!("scene_editor.glade");

    let builder = gtk::Builder::from_string(glade_src);

    let window: gtk::Window = builder.get_object("window_main").unwrap();
    let tree: gtk::TreeView = builder.get_object("scene_graph").unwrap();
    let store = TreeStore::new(&[String::static_type()]);

    tree.set_model(Some(&store));

    for i in 0..10 {
        let iter = store.insert_with_values(None, None, &[0], &[&format!("Hello {}", i)]);

        for _ in 0..i {
            store.insert_with_values(Some(&iter), None, &[0], &[&"I'm a child node"]);
        }
    }

    window.show_all();

    let universe = Universe::new();
    let mut editor_world = EditorWorld::new(universe.create_world());

    let root = editor_world.scene_graph.tree().root();

    for i in 0..4 {
        let handle = editor_world.add_entity(root, Child(i))?;
        let entity = editor_world.scene_graph.tree().value(handle).unwrap().clone();
        editor_world.add_component(entity, Empty)?;
        editor_world.add_component(entity, ChildComponent(i))?;
    }



    gtk::main();

    // let stdout = io::stdout().into_raw_mode()?;
    // let stdout = MouseTerminal::from(stdout);
    // let stdout = AlternateScreen::from(stdout);
    // let backend = TermionBackend::new(stdout);
    // let mut terminal = Terminal::new(backend)?;
    // terminal.hide_cursor()?;
    //
    // let events = Events::new();
    //
    //
    //
    //
    // let mut stateful_tree_list = StatefulTreeList::new(editor_world.scene_graph.tree());
    // let mut current: Option<TreeKey> = None;
    // let mut active: Option<TreeKey> = None;
    // loop {
    //     terminal.draw(|mut f| {
    //         let chunks = Layout::default()
    //             .direction(Direction::Vertical)
    //             .margin(1)
    //             .constraints(
    //                 [
    //                     Constraint::Percentage(85),
    //                     Constraint::Percentage(15),
    //                 ].as_ref()
    //             )
    //             .split(f.size());
    //
    //         // let block = Block::default()
    //         //     .title("Scene Graph")
    //         //     .borders(Borders::ALL);
    //
    //         let (tree_list, mut state) = stateful_tree_list.to_tree_list(editor_world.scene_graph.tree().root());
    //         let tree_list = tree_list
    //             .style(Style::default().fg(Color::White))
    //             .highlight_style(Style::default().modifier(Modifier::ITALIC))
    //             .indent_symbol("  ");
    //         current = match state.selected() {
    //             Some(i) => tree_list.get(i as usize),
    //             None => None
    //         };
    //
    //         f.render_stateful_widget(tree_list, chunks[0], &mut state);
    //
    //         let mut items = Vec::new();
    //         let mut name = "No entity selected";
    //         match active {
    //             Some(key) => {
    //                 let entity = editor_world.scene_graph.tree().value(key).unwrap();
    //                 let component = editor_world.world.get_component::<ComponentVisualization>(*entity).unwrap();
    //                 for comp in &component.components {
    //                     items.push(comp.name())
    //                 }
    //                 name = "test";
    //             }
    //             None => ()
    //         }
    //
    //         let list = List::new(items.iter().map(|i| Text::raw(i)))
    //             .block(Block::default().title(name).borders(Borders::ALL))
    //             .style(Style::default().fg(Color::White))
    //             .highlight_style(Style::default().modifier(Modifier::ITALIC))
    //             .highlight_symbol(">>");
    //
    //         f.render_widget(list, chunks[1]);
    //     })?;
    //
    //     match events.next()? {
    //         Event::Input(input) => match input {
    //             Key::Char('q') => {
    //                 break;
    //             }
    //             Key::Backspace => {
    //                 stateful_tree_list.unselect();
    //             }
    //             Key::Down => {
    //                 stateful_tree_list.next();
    //             }
    //             Key::Up => {
    //                 stateful_tree_list.previous();
    //             }
    //             Key::Right => {
    //                 match current {
    //                     Some(key) => {
    //                         stateful_tree_list.expand(key);
    //                     },
    //                     None => ()
    //                 }
    //             }
    //             Key::Left => {
    //                 match current {
    //                     Some(key) => {
    //                         stateful_tree_list.collapse(key);
    //                     },
    //                     None => ()
    //                 }
    //             }
    //             Key::Char('\n') => {
    //                 active = current
    //             }
    //             _ => {
    //                 eprintln!("{:?}", input)
    //             }
    //         }
    //         Event::Tick => {}
    //     }
    // }

    Ok(())
}
