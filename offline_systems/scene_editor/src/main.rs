mod editor_component;

use tui::{
    Terminal,
    backend::TermionBackend,
    widgets::{Block, Borders, BorderType},
    layout::{Layout, Constraint, Direction},
    style::{Style, Color, Modifier},
    widgets::{ListState, List, Text}
};

use termion::{
    raw::IntoRawMode,
    event::Key,
    input::{MouseTerminal, TermRead},
    screen::AlternateScreen
};

use std::{
    io,
    thread,
    time::Duration,
    sync::{
        mpsc,
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    iter,
};

use roest_runtime::{
    core_resources::scene_graph::SceneGraph
};

use legion::{
    prelude::*,
    storage::{
        Component
    },
    world::EntityMutationError
};
use crate::editor_component::{EditorComponent, ComponentRepresentation};
use roest_runtime::core_resources::tree::{TreeIndex, TreeError};
use crate::tree_list::TreeListState;

mod tree_list;

pub enum Event<I> {
    Input(I),
    Tick,
}

pub struct Empty;

pub struct ComponentVisualization {
    components: Vec<ComponentRepresentation>
}

pub struct EditorWorld {
    world: World,
    scene_graph: SceneGraph,
}

impl EditorWorld {
    pub fn new(mut world: World) -> Self {

        let comp_vis = ComponentVisualization {
            components: vec![Empty.as_component_representation()]
        };

        let root_entity = world.insert(
            (),
            iter::once(
                (Empty, comp_vis)
            )
        )[0];

        EditorWorld {
            world,
            scene_graph: SceneGraph::new(root_entity).unwrap()
        }
    }

    pub fn add_entity(&mut self, parent: TreeIndex) -> Result<TreeIndex, TreeError> {
        let comp_vis = ComponentVisualization {
            components: Vec::new()
        };
        let entity = self.world.insert(
            (),
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

impl EditorComponent for Empty {
    fn as_component_representation(&self) -> ComponentRepresentation {
        ComponentRepresentation::new("Empty".to_string())
    }
}

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
    rx: mpsc::Receiver<Event<Key>>,
    input_handle: thread::JoinHandle<()>,
    ignore_exit_key: Arc<AtomicBool>,
    tick_handle: thread::JoinHandle<()>,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub exit_key: Key,
    pub tick_rate: Duration,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            exit_key: Key::Char('q'),
            tick_rate: Duration::from_millis(250),
        }
    }
}

impl Events {
    pub fn new() -> Events {
        Events::with_config(Config::default())
    }

    pub fn with_config(config: Config) -> Events {
        let (tx, rx) = mpsc::channel();
        let ignore_exit_key = Arc::new(AtomicBool::new(false));
        let input_handle = {
            let tx = tx.clone();
            let ignore_exit_key = ignore_exit_key.clone();
            thread::spawn(move || {
                let stdin = io::stdin();
                for evt in stdin.keys() {
                    if let Ok(key) = evt {
                        if let Err(err) = tx.send(Event::Input(key)) {
                            eprintln!("{}", err);
                            return;
                        }
                        if !ignore_exit_key.load(Ordering::Relaxed) && key == config.exit_key {
                            return;
                        }
                    }
                }
            })
        };
        let tick_handle = {
            thread::spawn(move || loop {
                tx.send(Event::Tick).unwrap();
                thread::sleep(config.tick_rate);
            })
        };
        Events {
            rx,
            ignore_exit_key,
            input_handle,
            tick_handle,
        }
    }

    pub fn next(&self) -> Result<Event<Key>, mpsc::RecvError> {
        self.rx.recv()
    }

    pub fn disable_exit_key(&mut self) {
        self.ignore_exit_key.store(true, Ordering::Relaxed);
    }

    pub fn enable_exit_key(&mut self) {
        self.ignore_exit_key.store(false, Ordering::Relaxed);
    }
}

fn main() -> Result<(), failure::Error>{
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();

    let universe = Universe::new();
    let mut editor_world = EditorWorld::new(universe.create_world());

    let root = editor_world.scene_graph.root();

    for i in 0..4 {
        let handle = editor_world.add_entity(root)?;
        let entity = editor_world.scene_graph[handle];
        editor_world.add_component(entity, Empty);
    }

    let mut state = TreeListState::default();

    loop {
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(85),
                        Constraint::Percentage(15),
                    ].as_ref()
                )
                .split(f.size());

            let block = Block::default()
                .title("Scene Graph")
                .borders(Borders::ALL);

            let tree = editor_world.scene_graph.tree();

            let mut tree_list = tree_list::TreeList::from_tree(tree, root)
                .style(Style::default().fg(Color::White))
                .highlight_style(Style::default().modifier(Modifier::ITALIC))
                .indent_symbol("  ");

            f.render_stateful_widget(tree_list, chunks[0], &mut state);
            let block = Block::default()
                .title("Block 2")
                .borders(Borders::ALL);
            f.render_widget(block, chunks[1]);
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                Key::Left => {
                    state.select(None)
                }
                Key::Down => {
                    match state.selected(){
                        Some(i) => {
                            state.select(Some(i + 1))
                        }
                        None => state.select(Some(0))
                    };
                }
                Key::Up => {
                    match state.selected(){
                        Some(i) => {
                            state.select(Some(i - 1))
                        }
                        None => state.select(Some(0))
                    };
                }
                _ => {}
            }
            Event::Tick => {}
        }
    }

    Ok(())
}
