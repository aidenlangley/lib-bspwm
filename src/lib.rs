use serde::{Deserialize, Serialize};
use std::fmt::Display;

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
