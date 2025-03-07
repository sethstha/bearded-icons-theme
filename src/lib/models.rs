use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct IconTheme {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub name: String,
    pub author: String,
    pub themes: Vec<Theme>,
}

#[derive(Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub appearance: String,
    pub directory_icons: DirectoryIcons,
    pub file_stems: HashMap<String, String>,
    pub file_suffixes: HashMap<String, String>,
    pub file_icons: HashMap<String, FileIcon>,
}

#[derive(Serialize, Deserialize)]
pub struct DirectoryIcons {
    pub collapsed: String,
    pub expanded: String,
}

#[derive(Serialize, Deserialize)]
pub struct FileIcon {
    pub path: String,
}