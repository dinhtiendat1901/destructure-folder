use std::fs;
use std::path::PathBuf;

use tauri::command;

struct Line {
    path: PathBuf,
    is_folder: bool,
    is_last: bool,
    list_symbol: Vec<&'static str>,
}


impl Line {
    fn build_line(&self) -> String {
        let mut line = String::new();
        line.push_str("\n");
        for symbol in &self.list_symbol {
            line.push_str(symbol);
        }
        line.push_str(self.path.file_name().unwrap().to_str().unwrap());
        if self.is_folder {
            line.push_str("/");
        }
        line
    }
}

fn build_tree(line: Line, result: &mut String) -> &String {
    result.push_str(line.build_line().as_str());
    line.build_line();
    if let Ok(entries) = fs::read_dir(line.path) {
        let entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
        let last_index = entries.len() - 1;


        if line.is_folder {
            for (i, entry) in entries.into_iter().enumerate() {
                let path = entry.path();
                let is_last = i == last_index;
                let mut new_list_symbol = line.list_symbol.clone();
                if let Some(last) = new_list_symbol.last_mut() {
                    if line.is_last {
                        *last = "    "
                    } else {
                        *last = "│   "
                    }
                }
                let is_folder = if path.is_dir() { true } else { false };
                new_list_symbol.push(if is_last { "└── " } else { "├── " });
                build_tree(Line {
                    path,
                    is_folder,
                    is_last,
                    list_symbol: new_list_symbol,
                }, result);
            }
        }
    }
    result
}


#[command]
fn get_folder_structure(path: String) -> Result<String, String> {
    let mut binding = String::new();
    let result = build_tree(Line {
        path: PathBuf::from(path),
        is_folder: true,
        is_last: false,
        list_symbol: Vec::new(),
    }, &mut binding);

    let new_result = result.clone();
    Ok(new_result)
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_folder_structure])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
