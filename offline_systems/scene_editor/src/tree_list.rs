use std::{
    vec::{
        Vec,
        IntoIter
    },
    rc::Rc
};
use tui::{
    widgets::{
        StatefulWidget,
        ListState,
        List,
        Text,
        Widget
    },
    layout::Rect,
    buffer::Buffer,
};
use roest_runtime::core_resources::scene_graph::SceneGraph;
use roest_runtime::core_resources::tree::{TreeIndex, Tree};
use legion::prelude::Entity;
use tui::style::Style;

pub struct StatefulTreeList<'a> {
    tree: &'a Tree<Entity>,
    collapsed: Tree<bool>,
    state: TreeListState
}

impl StatefulTreeList {
    pub fn new(tree: &Tree<Entity>) -> Self {
        let collapsed = tree.map(|| false);
        StatefulTreeList {
            tree,
            collapsed,
            state: TreeListState::default()
        }
    }
}

pub struct TreeListItem<'a> {
    idx: TreeIndex,
    indent: u8,
    name: &'a str,
}

impl<'a> TreeListItem<'a> {
    pub fn new(idx: TreeIndex, indent: u8, name: &'a str) -> Self {
        TreeListItem {
            idx,
            indent,
            name
        }
    }

    pub fn to_string(&self, indent_char: &str) -> String {
        let padding = indent_char.repeat(self.indent as usize);
        format!("{}{}", padding, self.name)
    }
}

pub struct TreeList<'a> {
    items: Vec<TreeListItem<'a>>,
    style: Style,
    highlight_style: Style,
    highlight_symbol: Option<&'a str>,
    indent_symbol: &'a str
}

impl<'a> TreeList<'a> {
    pub fn from_tree(tree: &'a Tree<Entity>, root: TreeIndex) -> Self {
        TreeList {
            items: Self::dfs(tree, root, 0),
            style: Style::default(),
            highlight_style: Style::default(),
            highlight_symbol: None,
            indent_symbol: "    "
        }
    }

    pub fn from_stateful_tree(tree: &'a StatefulTreeList, root: TreeIndex) -> Self {

    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;

        self
    }

    pub fn highlight_style(mut self, highlight_style: Style) -> Self {
        self.highlight_style = highlight_style;

        self
    }

    pub fn highlight_symbol(mut self, highlight_symbol: &'a str) -> Self {
        self.highlight_symbol = Some(highlight_symbol);

        self
    }

    pub fn indent_symbol(mut self, indent_symbol: &'a str) -> Self {
        self.indent_symbol = indent_symbol;

        self
    }

    fn dfs_pruned(tree &Tree<Entity>, prune_mape: Tree<bool>)


    fn dfs(tree: &Tree<Entity>, node: TreeIndex, indent: u8) -> Vec<TreeListItem> {
        let mut out = Vec::new();
        out.push(TreeListItem::new(node, indent, "test"));

        match &tree[node] {
            Some(n) => {
                for c in n.get_children() {
                    out.extend(TreeList::dfs(tree, *c, indent + 1))
                }
            }
            None => (),
        }

        out
    }

    fn to_list(&self) -> List<IntoIter<Text>> {
        let mut item_names = Vec::with_capacity(self.items.len());
        for item in &self.items {
            item_names.push(Text::raw(item.to_string(self.indent_symbol)));
        }

        let list = match self.highlight_symbol {
            Some(symbol) => {
                List::new(item_names.into_iter())
                    .style(self.style)
                    .highlight_style(self.highlight_style)
                    .highlight_symbol(symbol)
            }
            None => {
                List::new(item_names.into_iter())
                    .style(self.style)
                    .highlight_style(self.highlight_style)
            }
        };

        list
    }
}

impl<'a> Widget for TreeList<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Widget::render(self.to_list(), area, buf)
    }
}

impl<'a> StatefulWidget for TreeList<'a> {
    type State = TreeListState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        StatefulWidget::render(self.to_list(), area, buf, &mut state.to_list_state())
    }
}

pub struct TreeListState {
    offset: usize,
    selected: Option<usize>,
}

impl TreeListState {
    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.selected = index;
        if index.is_none() {
            self.offset = 0;
        }
    }

    pub fn to_list_state(&self) -> ListState {
        let mut state = ListState::default();
        state.select(self.selected);

        state
    }

    // pub fn next(&mut self) {
    //     let i = match self.selected {
    //         Some(i) => {
    //             if i >= self.items.len() {
    //                 0
    //             } else {
    //                 i + 1
    //             }
    //         }
    //         None => 0,
    //     };
    //
    //     self.state.select(Some(i));
    // }
    //
    // pub fn previous(&mut self) {
    //     let i = match self.selected {
    //         Some(i) => {
    //             if i <= 0 {
    //                 self.items.len() - 1
    //             } else {
    //                 i - 1
    //             }
    //         }
    //         None => 0,
    //     };
    //
    //     self.state.select(Some(i));
    // }
    //
    // pub fn unselect(&mut self) {
    //     self.state.select(None)
    // }
}

impl Default for TreeListState {
    fn default() -> Self {
        TreeListState {
            offset: 0,
            selected: None
        }
    }
}