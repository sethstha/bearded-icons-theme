mod lib;

use lib::test_generator::generate_test_files;
use lib::theme_config::generate_theme_config;

fn main() {
    println!("Generating theme configuration...");

    if let Err(e) = generate_theme_config() {
        eprintln!("Error generating theme configuration: {}", e);
    }

    println!("Generating test files...");
    if let Err(e) = generate_test_files() {
        eprintln!("Error generating test files: {}", e);
    }

    println!("Done!");
}
