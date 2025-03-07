use std::fs::{self};
use std::io;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let release_dir = Path::new("release");
    let excluded_items = vec![
        "tests",
        "Cargo.toml",
        "Cargo.lock",
        ".gitignore",
        "target",
        "release",
        "build.rs",
        ".git",
        ".vscode",
    ];

    // Clean up release directory if it exists
    if release_dir.exists() {
        fs::remove_dir_all(release_dir)?;
    }
    // Create fresh release directory
    fs::create_dir_all(release_dir)?;

    // First, ensure icon_themes directory exists
    fs::create_dir_all("icon_themes")?;

    // Copy files and folders
    copy_items(Path::new("."), release_dir, &excluded_items)?;

    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=icons");
    Ok(())
}

fn copy_items(src: &Path, dst: &Path, excluded: &[&str]) -> io::Result<()> {
    if excluded.contains(&src.file_name().and_then(|n| n.to_str()).unwrap_or("")) {
        return Ok(());
    }

    if src.is_dir() {
        match fs::create_dir_all(dst) {
            Ok(_) => {}
            Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
                eprintln!(
                    "Permission denied when creating directory {}: {}",
                    dst.display(),
                    e
                );
                return Err(e);
            }
            Err(e) => {
                eprintln!("Failed to create directory {}: {}", dst.display(), e);
                return Err(e);
            }
        }

        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            let new_dst = dst.join(entry.file_name());

            if file_type.is_dir() {
                copy_items(&entry.path(), &new_dst, excluded)?
            } else {
                if !excluded.contains(&entry.file_name().to_str().unwrap_or("")) {
                    match fs::copy(entry.path(), &new_dst) {
                        Ok(_) => {}
                        Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
                            eprintln!(
                                "Permission denied when copying file {}: {}",
                                entry.path().display(),
                                e
                            );
                            return Err(e);
                        }
                        Err(e) => {
                            eprintln!("Failed to copy file {}: {}", entry.path().display(), e);
                            return Err(e);
                        }
                    }
                }
            }
        }
    } else {
        match fs::copy(src, dst) {
            Ok(_) => {}
            Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
                eprintln!(
                    "Permission denied when copying file {}: {}",
                    src.display(),
                    e
                );
                return Err(e);
            }
            Err(e) => {
                eprintln!("Failed to copy file {}: {}", src.display(), e);
                return Err(e);
            }
        }
    }

    Ok(())
}
