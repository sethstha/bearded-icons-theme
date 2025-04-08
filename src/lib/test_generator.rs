use std::fs;
use std::io::Write;
use std::path::Path;

use super::file_handlers::{load_file_stems, load_file_suffixes};

pub fn generate_test_files() -> Result<(), Box<dyn std::error::Error>> {
    // Load file configurations
    let file_stems = load_file_stems()?;
    let file_suffixes = load_file_suffixes()?;

    // Create test directory if it doesn't exist
    let test_dir = Path::new("tests");
    fs::create_dir_all(test_dir)?;

    // Generate files from stems
    for (file_name, _icon) in file_stems {
        let file_path = test_dir.join(&file_name);
        let mut file = fs::File::create(file_path)?;
        write!(file, "Test file for {}", file_name)?;
    }

    // Generate files from suffixes
    for (extension, _icon) in file_suffixes {
        let file_name = format!("test.{}", extension);
        let file_path = test_dir.join(file_name);
        let mut file = fs::File::create(file_path)?;
        write!(file, "Test file for extension {}", extension)?;
    }

    println!("Test files generated successfully!");
    Ok(())
}
