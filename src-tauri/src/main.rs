use ignore::WalkBuilder;
use std::path::{Path, PathBuf};
use tauri::command;

#[command]
fn get_folder_structure(path: String) -> Result<String, String> {
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
                    tree.push_str("\n");
                    tree.push_str(final_path.as_str());
                }
            }
            Err(err) => eprintln!("Error: {}", err),
        }
    }
    Ok(tree)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_folder_structure])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
