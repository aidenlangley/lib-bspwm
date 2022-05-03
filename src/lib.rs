use serde::{Deserialize, Serialize};
use std::{fmt::Display, process::Command};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Layout {
    Tiled,
    PsuedoTiled,
    Floating,
    Monocle,
}

impl Default for Layout {
    fn default() -> Self {
        Self::Tiled
    }
}

impl Display for Layout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tiled => write!(f, "tiled"),
            Self::PsuedoTiled => write!(f, "pseudo-tiled"),
            Self::Floating => write!(f, "floating"),
            Self::Monocle => write!(f, "monocle"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SplitType {
    Vertical,
    Horizontal,
}

impl Default for SplitType {
    fn default() -> Self {
        Self::Vertical
    }
}

impl Display for SplitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SplitType::Vertical => write!(f, "vertical"),
            SplitType::Horizontal => write!(f, "horizontal"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Layer {
    Normal,
}

impl Default for Layer {
    fn default() -> Self {
        Self::Normal
    }
}

impl Display for Layer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Layer::Normal => write!(f, "normal"),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Constraints {
    pub min_width: usize,
    pub min_height: usize,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Padding {
    pub top: usize,
    pub right: usize,
    pub bottom: usize,
    pub left: usize,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rectangle {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Focus {
    monitor_id: usize,
    desktop_id: usize,
    node_id: usize,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub focused_monitor_id: usize,
    pub primary_monitor_id: usize,
    pub clients_count: usize,
    pub monitors: Vec<Monitor>,
    pub focus_history: Vec<Focus>,
    pub stacking_list: Vec<usize>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Monitor {
    pub name: String,
    pub id: usize,
    pub randr_id: usize,
    pub wired: bool,
    pub sticky_count: usize,
    pub window_gap: usize,
    pub border_width: usize,
    pub focused_desktop_id: usize,
    pub desktops: Vec<Desktop>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Desktop {
    pub name: String,
    pub id: usize,
    pub layout: Layout,
    pub user_layout: Layout,
    pub window_gap: usize,
    pub border_width: usize,
    pub focused_node_id: usize,
    pub padding: Padding,
    pub root: Option<Tree>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tree {
    pub id: usize,
    pub split_type: SplitType,
    pub vacant: bool,
    pub hidden: bool,
    pub sticky: bool,
    pub private: bool,
    pub locked: bool,
    pub marked: bool,
    pub presel: Option<String>,
    pub rectangle: Rectangle,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: usize,
    pub split_type: SplitType,
    pub split_ratio: f32,
    pub vacant: bool,
    pub hidden: bool,
    pub sticky: bool,
    pub private: bool,
    pub locked: bool,
    pub marked: bool,
    pub presel: Option<String>,
    pub rectangle: Rectangle,
    pub constrainnts: Option<Constraints>,
    pub first_child: Option<Box<Node>>,
    pub second_child: Option<Box<Node>>,
    pub client: Client,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Client {
    pub class_name: String,
    pub instance_name: String,
    pub border_width: usize,
    pub state: Layout,
    pub last_state: Layout,
    pub layer: Layer,
    pub last_layer: Layer,
    pub urgent: bool,
    pub shown: bool,
    pub tiled_rectangle: Rectangle,
    pub floating_rectangle: Rectangle,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to deserialize output from `bspc wm -d` or `bspc query <DOMAIN>`")]
    Deserialize,

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

const PROG: &'static str = "bspc";
const DUMP_ARGS: &'static [&'static str; 2] = &["wm", "-d"];
const QUERY_ARGS: &'static [&'static str; 1] = &["query"];
const QUERY_TREE_FLAG: &'static str = "-T";
const MON_SEL_QUERY_FLAG: &'static str = "-m";
const DESK_SEL_QUERY_FLAG: &'static str = "-d";
const NODE_SEL_QUERY_FLAG: &'static str = "-n";
const _MON_ID_OUT_FLAG: &'static str = "-M";
const _DESK_ID_OUT_FLAG: &'static str = "-D";
const _NODE_ID_OUT_FLAG: &'static str = "-N";

pub fn dump_state() -> Result<State, self::Error> {
    match serde_json::from_slice::<State>(&Command::new(PROG).args(DUMP_ARGS).output()?.stdout) {
        Ok(state) => Ok(state),
        Err(_) => Err(self::Error::Deserialize),
    }
}

#[derive(Debug)]
pub struct Query {
    args: Vec<String>,
}

impl Query {
    pub fn new() -> Self {
        Self {
            args: QUERY_ARGS.iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn monitor(mut self, name: &str) -> Self {
        if name.is_empty() {
            return self;
        }

        // Make a new Vec so that we're aren't passing multiple domain selectors.
        // `bspc` doesn't like it.
        let mut args: Vec<String> = QUERY_ARGS.iter().map(|s| s.to_string()).collect();
        args.push(MON_SEL_QUERY_FLAG.to_string());
        args.push(name.to_string());
        self.args = args;

        self
    }

    pub fn desktop(mut self, name: &str) -> Self {
        if name.is_empty() {
            return self;
        }

        let mut args: Vec<String> = QUERY_ARGS.iter().map(|s| s.to_string()).collect();
        args.push(DESK_SEL_QUERY_FLAG.to_string());
        args.push(name.to_string());
        self.args = args;

        self
    }

    pub fn node(mut self, id: usize) -> Self {
        let mut args: Vec<String> = QUERY_ARGS.iter().map(|s| s.to_string()).collect();
        args.push(NODE_SEL_QUERY_FLAG.to_string());
        args.push(id.to_string());
        self.args = args;

        self
    }

    fn _get_monitor_ids(&mut self) -> Vec<usize> {
        self.args.push(_MON_ID_OUT_FLAG.to_string());
        todo!()
    }

    fn _get_desktop_ids(&mut self) -> Vec<usize> {
        self.args.push(_DESK_ID_OUT_FLAG.to_string());
        todo!()
    }

    fn _get_node_ids(&mut self) -> Vec<usize> {
        self.args.push(_NODE_ID_OUT_FLAG.to_string());
        todo!()
    }

    pub fn get_monitor_tree(&mut self) -> Result<Monitor, self::Error> {
        // `bspc` requires at least one domain flag, even if it's passed
        // parameter-less, so here we count the flags and add a single `-m` if
        // necessary.
        if self.args.len() == 1 {
            self.args.push(MON_SEL_QUERY_FLAG.to_string());
        }

        self.args.push(QUERY_TREE_FLAG.to_string());
        match serde_json::from_slice::<Monitor>(
            &Command::new(PROG).args(&self.args).output()?.stdout,
        ) {
            Ok(m) => Ok(m),
            Err(_) => Err(self::Error::Deserialize),
        }
    }

    pub fn get_desktop_tree(&mut self) -> Result<Desktop, self::Error> {
        if self.args.len() == 1 {
            self.args.push(DESK_SEL_QUERY_FLAG.to_string());
        }

        self.args.push(QUERY_TREE_FLAG.to_string());
        match serde_json::from_slice::<Desktop>(
            &Command::new(PROG).args(&self.args).output()?.stdout,
        ) {
            Ok(d) => Ok(d),
            Err(_) => Err(self::Error::Deserialize),
        }
    }

    pub fn get_node_tree(&mut self) -> Result<Node, self::Error> {
        if self.args.len() == 1 {
            self.args.push(NODE_SEL_QUERY_FLAG.to_string());
        }

        self.args.push(QUERY_TREE_FLAG.to_string());
        match serde_json::from_slice::<Node>(&Command::new(PROG).args(&self.args).output()?.stdout)
        {
            Ok(n) => Ok(n),
            Err(_) => Err(self::Error::Deserialize),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    use crate::{dump_state, Query, DUMP_ARGS, PROG, QUERY_ARGS};

    #[test]
    fn dump_args() {
        let mut args = vec![PROG];
        for arg in DUMP_ARGS {
            args.push(arg)
        }

        assert_eq!(args, vec![PROG, "wm", "-d"])
    }

    #[test]
    fn query_args() {
        let mut args = vec![PROG];
        for arg in QUERY_ARGS {
            args.push(arg)
        }

        assert_eq!(args, vec![PROG, "query"])
    }

    #[test]
    fn outputs() {
        if let Err(e) = Command::new(PROG).args(DUMP_ARGS).output() {
            assert!(false, "{}", e)
        }
    }

    #[test]
    fn deserializes() {
        if let Err(e) = dump_state() {
            assert!(false, "{}", e)
        }
    }

    #[test]
    fn monitor_tree() {
        let mut query = Query::new();
        if let Err(e) = query.get_monitor_tree() {
            eprintln!("{:#?}", query);
            assert!(false, "{}", e)
        }
    }

    #[test]
    fn desktop_tree() {
        let mut query = Query::new();
        if let Err(e) = query.get_desktop_tree() {
            eprintln!("{:#?}", query);
            assert!(false, "{}", e)
        }
    }

    #[test]
    fn node_tree() {
        let mut query = Query::new();
        if let Err(e) = query.get_node_tree() {
            eprintln!("{:#?}", query);
            assert!(false, "{}", e)
        }
    }

    #[test]
    fn with_param() {
        let query = Query::new().monitor("testies");
        assert_eq!(query.args, vec!["query", "-m", "testies"])
    }

    #[test]
    fn try_many_param() {
        let query = Query::new().monitor("discarded").monitor("testies");
        assert_eq!(query.args, vec!["query", "-m", "testies"])
    }
}
