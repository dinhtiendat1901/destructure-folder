use std::fs;
use std::path::PathBuf;

struct Line {
    path: PathBuf,
    is_folder: bool,
    is_last: bool,
    list_symbol: Vec<&'static str>,
}


impl Line {
    fn print_line(&self) {
        let mut line = String::new();
        for symbol in &self.list_symbol {
            line.push_str(symbol);
        }
        line.push_str(self.path.file_name().unwrap().to_str().unwrap());
        if self.is_folder {
            line.push_str("/");
        }
        println!("{}", line);
    }
}

fn build_tree(line: Line) {
    line.print_line();
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
                });
            }
        }
    }
}



fn main() {
    build_tree(Line {
        path: PathBuf::from(r"C:\Users\dinhtiendat1901\Desktop\New folder"),
        is_folder: true,
        is_last: false,
        list_symbol: Vec::new(),
    })
}
