use std::collections::HashMap;
use std::fs;
use std::path::Path;
use toml;

pub fn load_file_stems() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut file_stems = HashMap::new();
    let names_dir = Path::new("src/names");
    
    // Process bundler.toml
    let bundler_path = names_dir.join("bundler.toml");
    let content = fs::read_to_string(bundler_path)?;
    let toml_value: toml::Value = toml::from_str(&content)?;

    if let toml::Value::Table(table) = toml_value {
        for (section_name, section_value) in table {
            if let toml::Value::Table(section_table) = section_value {
                if let Some(toml::Value::Array(files)) = section_table.get("files") {
                    for file in files {
                        if let toml::Value::String(file_name) = file {
                            file_stems.insert(file_name.clone(), section_name.clone());
                        }
                    }
                }
            }
        }
    }

    // Process file_names.toml
    let file_names_path = names_dir.join("file_names.toml");
    let content = fs::read_to_string(file_names_path)?;
    let toml_value: toml::Value = toml::from_str(&content)?;

    if let toml::Value::Table(table) = toml_value {
        for (file_name, icon_name) in table {
            if let toml::Value::String(icon) = icon_name {
                file_stems.insert(file_name, icon);
            }
        }
    }

    Ok(file_stems)
}

pub fn load_file_suffixes() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut file_suffixes = HashMap::new();
    let extensions_dir = Path::new("src/extensions");

    // Read and parse file_extensions.toml
    let file_extensions_path = extensions_dir.join("file_extensions.toml");
    let content = fs::read_to_string(file_extensions_path)?;
    let toml_value: toml::Value = toml::from_str(&content)?;

    if let toml::Value::Table(table) = toml_value {
        for (extension, icon_name) in table {
            if let toml::Value::String(icon) = icon_name {
                file_suffixes.insert(extension, icon);
            }
        }
    }

    // Read and parse media.toml
    let media_path = extensions_dir.join("media.toml");
    let content = fs::read_to_string(media_path)?;
    let toml_value: toml::Value = toml::from_str(&content)?;

    if let toml::Value::Table(table) = toml_value {
        for (media_type, media_value) in table {
            if let toml::Value::Table(media_table) = media_value {
                if let Some(toml::Value::Array(types)) = media_table.get("types") {
                    for type_value in types {
                        if let toml::Value::String(extension) = type_value {
                            file_suffixes.insert(extension.clone(), media_type.clone());
                        }
                    }
                }
            }
        }
    }

    Ok(file_suffixes)
}