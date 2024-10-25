#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use ignore::WalkBuilder;
use serde::Serialize;
use tauri::command;

#[derive(Serialize)]
struct FileInfo {
    path: String,
    content: String,
}

#[command]
fn get_folder_structure(path: String) -> Result<(String, String, Vec<FileInfo>), String> {
    let tree = build_file_path(path.clone())?;
    let (content, file_info_list) = build_file_content(path)?;
    Ok((tree, content, file_info_list))
}

fn build_file_path(path: String) -> Result<String, String> {
    let mut tree = String::new();

    let base_path = Path::new(path.as_str());
    let base_path_name = base_path.file_name().unwrap().to_string_lossy();

    let walker = WalkBuilder::new(base_path).standard_filters(true).build();

    for result in walker {
        match result {
            Ok(entry) => {
                if let Ok(relative_path) = entry.path().strip_prefix(base_path) {
                    let formatted_path: PathBuf = relative_path.iter().collect();
                    let final_path = format!(
                        "{}/{}",
                        base_path_name,
                        formatted_path.display().to_string().replace("\\", "/")
                    );
                    tree.push_str(&format!("\n{}", final_path));
                }
            }
            Err(err) => eprintln!("Error: {}", err),
        }
    }
    tree.push_str("\n");
    Ok(tree)
}

fn build_file_content(path: String) -> Result<(String, Vec<FileInfo>), String> {
    let mut content = String::new();
    let mut file_info_list = Vec::new();

    let base_path = Path::new(path.as_str());
    let base_path_name = base_path.file_name().unwrap().to_string_lossy();

    let walker = WalkBuilder::new(base_path).standard_filters(true).build();

    for result in walker {
        match result {
            Ok(entry) => {
                if let Ok(relative_path) = entry.path().strip_prefix(base_path) {
                    let formatted_path: PathBuf = relative_path.iter().collect();
                    let final_path = format!(
                        "{}/{}",
                        base_path_name,
                        formatted_path.display().to_string().replace("\\", "/")
                    );

                    if entry.path().is_file() {
                        content.push_str(&format!("\nContent of file in path {}:\n", final_path));
                        let mut file_content = String::new();
                        match File::open(entry.path()) {
                            Ok(mut file) => {
                                if file.read_to_string(&mut file_content).is_ok() {
                                    content.push_str(file_content.as_str());
                                    content.push_str("\n\n\n");
                                    file_info_list.push(FileInfo {
                                        path: final_path.clone(),
                                        content: file_content.clone(),
                                    });
                                } else {}
                            }
                            Err(_) => {}
                        }
                    }
                }
            }
            Err(err) => eprintln!("Error: {}", err),
        }
    }
    Ok((content, file_info_list))
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![get_folder_structure])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
