use std::fs;
use std::path::Path;
use super::models::{IconTheme, Theme, DirectoryIcons, FileIcon};
use super::file_handlers::{load_file_stems, load_file_suffixes};
use std::collections::HashMap;

pub fn generate_theme_config() -> Result<(), Box<dyn std::error::Error>> {
    let mut theme = IconTheme {
        schema: "https://zed.dev/schema/icon_themes/v0.2.0.json".to_string(),
        name: "Bearded Icon Theme".to_string(),
        author: "Sanjeev Shrestha".to_string(),
        themes: vec![Theme {
            name: "Bearded Icon Theme".to_string(),
            appearance: "dark".to_string(),
            directory_icons: DirectoryIcons {
                collapsed: "./icons/folders/folder.svg".to_string(),
                expanded: "./icons/folders/folder_open.svg".to_string(),
            },
            file_stems: load_file_stems().unwrap_or_default(),
            file_suffixes: load_file_suffixes().unwrap_or_default(),
            file_icons: HashMap::new(),
        }],
    };

    let icons_dir = Path::new("icons");
    let mut file_icons = HashMap::new();

    for entry in fs::read_dir(icons_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().and_then(|s| s.to_str()) == Some("svg") {
            let file_name = path.file_stem()
                .and_then(|s| s.to_str())
                .ok_or("Invalid file name")?;
                
            file_icons.insert(
                file_name.to_string(),
                FileIcon {
                    path: format!("./icons/{}.svg", file_name),
                },
            );
        }
    }

    if let Some(first_theme) = theme.themes.get_mut(0) {
        first_theme.file_icons = file_icons;
    }

    let output_file = Path::new("icon_themes/bearded-icon-theme.json");
    fs::write(
        output_file,
        serde_json::to_string_pretty(&theme)?
    )?;

    println!("Theme configuration has been generated successfully!");
    Ok(())
}