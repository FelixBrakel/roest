use std::{
    vec::{
        Vec,
        IntoIter
    },
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
use legion::prelude::Entity;
use tui::style::Style;
use slotted_tree::{
    TreeKey,
    SecondaryTree,
    Tree
};

pub struct StatefulTreeList<'a> {
    tree: &'a Tree<Entity>,
    collapsed: SecondaryTree<bool>,
    state: TreeListState
}

impl<'a> StatefulTreeList<'a> {
    pub fn new(tree: &'a Tree<Entity>) -> Self {
        let collapsed = tree.map_to_secondary(|_, _| false);
        StatefulTreeList {
            tree,
            collapsed,
            state: TreeListState::default()
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                    i + 1
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                i - 1
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }

    pub fn to_tree_list(&mut self, root: TreeKey) -> (TreeList, TreeListState) {
        let items = self.pruned_dfs(root, 0);

        let len = items.len();
        let tree_list = TreeList::from_items(items);
        match self.state.selected() {
            Some(i) => {
                if i > (len - 1) as i32 {
                    self.state.select(Some(0));
                } else if i < 0 {
                    self.state.select(Some((len - 1) as i32))
                }
            }
            None => ()
        }

        (tree_list, self.state)
    }

    fn pruned_dfs(&self, node: TreeKey, indent: u8) -> Vec<TreeListItem> {
        let mut out = Vec::new();


        let prune_node = match self.collapsed.get(node) {
            Some(b) => { *b },
            None => false
        };

        let len = self.tree.children(node).unwrap().len();

        let mut collapsed_symb = "";
        if len > 0 {
            if prune_node {
                collapsed_symb = "▶"
            } else {
                collapsed_symb = "▼"
            }
        }

        out.push(TreeListItem::new(node, indent, format!("{}test", collapsed_symb)));


        if !prune_node {
            match self.tree.children(node) {
                Some(children) => {
                    for c in children {
                        out.extend(self.pruned_dfs(*c, indent + 1))
                    }
                }
                None => (),
            }
        }

        out
    }

    pub fn collapse(&mut self, node: TreeKey) {
        self.collapsed.insert(node, true);
    }

    pub fn expand(&mut self, node: TreeKey) {
        self.collapsed.insert(node, false);
    }
}

pub struct TreeListItem {
    idx: TreeKey,
    indent: u8,
    name: String,
}

impl TreeListItem {
    pub fn new<T: AsRef<str>>(idx: TreeKey, indent: u8, name: T) -> Self {
        TreeListItem {
            idx,
            indent,
            name: name.as_ref().to_string()
        }
    }

    pub fn to_string(&self, indent_char: &str) -> String {
        let padding = indent_char.repeat(self.indent as usize);
        format!("{}{}", padding, self.name)
    }
}

pub struct TreeList<'a> {
    items: Vec<TreeListItem>,
    style: Style,
    highlight_style: Style,
    highlight_symbol: Option<&'a str>,
    indent_symbol: &'a str
}

impl<'a> TreeList<'a> {
    pub fn get(&self, idx: usize) -> Option<TreeKey> {
        match self.items.get(idx) {
            Some(item) => Some(item.idx),
            None => None
        }
    }

    #[allow(dead_code)]
    pub fn from_tree(tree: &'a Tree<Entity>, root: TreeKey) -> Self {
        TreeList {
            items: Self::dfs(tree, root, 0),
            style: Style::default(),
            highlight_style: Style::default(),
            highlight_symbol: None,
            indent_symbol: "    "
        }
    }

    pub fn from_items(items: Vec<TreeListItem>) -> Self {
        TreeList {
            items,
            style: Style::default(),
            highlight_style: Style::default(),
            highlight_symbol: None,
            indent_symbol: "    "
        }
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;

        self
    }

    pub fn highlight_style(mut self, highlight_style: Style) -> Self {
        self.highlight_style = highlight_style;

        self
    }

    #[allow(dead_code)]
    pub fn highlight_symbol(mut self, highlight_symbol: &'a str) -> Self {
        self.highlight_symbol = Some(highlight_symbol);

        self
    }

    pub fn indent_symbol(mut self, indent_symbol: &'a str) -> Self {
        self.indent_symbol = indent_symbol;

        self
    }

    fn dfs(tree: &Tree<Entity>, node: TreeKey, indent: u8) -> Vec<TreeListItem> {
        let mut out = Vec::new();
        out.push(TreeListItem::new(node, indent, "test"));

        let ch = tree.children(node).unwrap();
        for c in ch {
            out.extend(TreeList::dfs(tree, *c, indent + 1))
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

#[derive(Copy, Clone)]
pub struct TreeListState {
    offset: usize,
    selected: Option<i32>,
}

impl TreeListState {
    pub fn selected(&self) -> Option<i32> {
        self.selected
    }

    pub fn select(&mut self, index: Option<i32>) {
        self.selected = index;
        if index.is_none() {
            self.offset = 0;
        }
    }

    pub fn to_list_state(&self) -> ListState {
        let mut state = ListState::default();
        state.select(match self.selected {
            Some(i) => {
                Some(i as usize)
            }
            None => None
        });

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