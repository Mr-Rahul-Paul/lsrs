use clap::Parser;
use owo_colors::OwoColorize;
use std::{
    fs::{self},
    // fs::{self, File},
    path::{Path, PathBuf},
};
use strum_macros::Display;
use tabled::{
    Table, Tabled,
    settings::{Color, Style, object::Columns},
};

#[derive(Debug, Parser)]
#[command(/*-V*/version = "1.0", author = "Paul", /*-h and --help*/about="testing",long_about = "A simple CLI tool")]
struct Cli {
    path: Option<PathBuf>,
}

#[derive(Debug, Display)]
enum EntryType {
    File,
    Dir,
}
#[derive(Debug, Tabled)]
struct FileEntry {
    name: String,
    #[tabled{rename="Type"}]
    e_type: EntryType,
    len_bytes: u64,
    modified: String,
}
fn main() {
    let cli = Cli::parse();
    let path = cli.path.unwrap_or(PathBuf::from("."));

    if let Ok(does_exist) = fs::exists(&path) {
        if does_exist {
            let get_files = get_files(&path);
            let mut table = Table::new(get_files);
            table.with(Style::rounded());
            table.modify(Columns::first(), Color::FG_BRIGHT_YELLOW);
            table.modify(Columns::one(1), Color::FG_BRIGHT_CYAN);
            table.modify(Columns::one(2), Color::FG_BRIGHT_BLUE);
            print!("{}", table);
        } else {
            println!("{}", "path doesnt exist".red());
        }
    } else {
        println!("{}", "error checking path".red());
        // return;
    }
    //comment to check if the path is correctly parsed
    // print!("\n\nworking?? path info -- {:?}\n", path);
}

fn get_files(path: &Path) -> Vec<FileEntry> {
    let mut data = Vec::default();
    if let Ok(read_dir) = fs::read_dir(path) {
        for entry in read_dir {
            if let Ok(file) = entry {
                if let Ok(meta) = fs::metadata(&file.path()) {
                    data.push(FileEntry {
                        name: file
                            .file_name()
                            .into_string()
                            .unwrap_or("unknown name".into()),
                        e_type: if meta.is_dir() {
                            EntryType::Dir
                        } else {
                            EntryType::File
                        },
                        len_bytes: meta.len(),
                        modified: "".to_string(), // placeholder for now
                    });
                }
            }
        }
    }
    data
}
