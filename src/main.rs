use std::fs;
use std::fs::{ReadDir, DirEntry};
use std::path::Path;
use structopt::StructOpt;
use hag::md_helper;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(
    short = "d",
    help = "Target dir to search for articles"
    )]
//    target_dir: Option<String>,
    target_dir: String,
}

fn main() {

    // read args
    let args = Cli::from_args();
    let target_dir = args.target_dir;
    println!("Hello, world! in dir {:?}", &target_dir);

    // TODO temp test md_helper
    let md_url = md_helper::make_markdown_url("test_text", &target_dir);
    println!("Hugo link: {:?}", &md_url);

    // build target path
    let base_path = Path::new(".")
        .join("..")
        .join("..")
        .join("content");

    let highlights_path = base_path
        .join("highlights");

    // target year dir
    let highlights_path = highlights_path
        .join("2020");
    // TODO get most recent numeric dir name as default if no arg provided

    // temp display path
    let full_path = fs::canonicalize(&highlights_path).unwrap();
    println!("Căutare fișiere în {}", full_path.display());

    // citire directoare si filtrare nume compuse numai din numere (lunile)
    let list_of_files: ReadDir = fs::read_dir(&highlights_path).unwrap();
    let numeric_dirs = list_of_files
        .filter(|file|
            file_name_is_numeric(file.as_ref().unwrap())
        );

    for dir in numeric_dirs {
        println!("{}", "-".repeat(50));
        walk_dir(&dir.unwrap(), 0);
    }
}

// citeste recursiv directoarele si afiseaza dirs si fisiere
// TODO pass function as argument to call on each file
fn walk_dir(dir: &DirEntry, iteration: usize) {
    if dir.path().is_dir() {
        println!("{}[{}]", " ".repeat(iteration*2), &dir.file_name().to_str().unwrap());
        let dir_files: ReadDir = fs::read_dir(&dir.path()).unwrap();
        for new_dir_res in dir_files {
            let new_dir: DirEntry = new_dir_res.unwrap();
            walk_dir(&new_dir, iteration + 1)
        }
    } else {
        if dir.file_name().to_str().unwrap() == "index.md" {
            // TODO read index.md file
            // citeste sectiunea si subsectiunea cautata -> get linii
            // update lista de linii de returnat
            println!("{}- {}", " ".repeat(iteration*2), dir.file_name().to_str().unwrap());
        } else {
            println!("{}- {}", " ".repeat(iteration*2), "other file, don't care");
        }
    }
}

fn file_name_is_numeric(filename: &DirEntry) -> bool {
//    filename.file_name().to_str().unwrap().parse::<u32>().is_ok()
    filename.file_name().to_str().unwrap().chars().all(char::is_numeric)
}

fn citeste_fisier() -> () {
    unimplemented!()
}

fn scrie_fisier() -> () {
    unimplemented!()
}

fn genereaza_cuprins() -> () {
    // TODO nice to have
    unimplemented!()
}