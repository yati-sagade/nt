use std::env;
use std::os;
use std::process::Command;
use rand::Rng;
use rand;
use std::io::Read;
use std::fs;
use std::path::Path;
use std::fs::File;
use super::Note;

pub fn edit_note(note: &Note) -> Option<Note> {
    let editor = get_editor();
    let path = tmp_file_path();
    let output = Command::new(&editor)
                         .arg(path.to_str().unwrap())
                         .output()
                         .unwrap();

    let mut f = File::open(path).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    fs::remove_file(&path);
    println!("{}", s);
    None
}

fn get_editor() -> String {
    let terminal_is_dumb = match env::var("TERM") {
        Ok(val) => val == "dumb",
        Err(_) => true,
    };
    if !terminal_is_dumb {
        match env::var("VISUAL") {
            Ok(name) => { return name; },
            Err(_) => { },
        }
    } 
    match env::var("EDITOR") {
        Ok(name) => { return name; },
        Err(_) => { },
    }
    let mut default_editor = String::new();
    default_editor.push_str("vi");
    default_editor
}

fn tmp_file_path() -> Path {
    let mut rng = rand::thread_rng();
    let filename: String = rng.gen_ascii_chars().take(10).collect();
    let mut path = os::tmpdir().to_path_buf();
    path.push(filename);
    *path.as_path()
}

