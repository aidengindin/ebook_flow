use std::{fs, collections::HashSet};

#[derive(Eq, Hash, PartialEq)]
struct BookFile {
    path: String,
    file_type: FileType,
}

impl BookFile {
    fn new(path: String, file_type: FileType) -> BookFile {
        BookFile { path, file_type }
    }
}

#[derive(Eq, Hash, PartialEq)]
enum FileType {
    Acsm,
    Epub,
    Azw,
}

fn main() {
    let paths = fs::read_dir("./").unwrap();
    let mut acsm_files = HashSet::new();
    let mut ebook_files = HashSet::new();

    for path in paths {
        let path = path.unwrap().path().display().to_string();
        if path.contains(".acsm") {
            acsm_files.insert(BookFile::new(path, FileType::Acsm));
        } else if path.contains(".epub") {
            ebook_files.insert(BookFile::new(path, FileType::Epub));
        } else if path.contains(".azw") {
            ebook_files.insert(BookFile::new(path, FileType::Azw));
        }
    }

    println!("{} acsm files", acsm_files.len());
    println!("{} ebook files", ebook_files.len());
}
