use std::env;
use std::os;
use std::process::{Command,Stdio};
use rand::Rng;
use rand;
use std::io::{Read,Write,BufReader,BufRead};
use std::fs;
use std::path::{Path,PathBuf};
use std::fs::File;
use super::Note;
use super::util;
use std::io::Result;

pub fn edit_note(note: &Note) -> Result<Note> {
    let editor = get_editor(); // is "vi"
    println!("Have the editor as {}", editor);
    let path = tmp_file_path();
    println!("Have the path as {:?}", path);

    {
        let mut fp = try!(File::create(&path));
        try!(fp.write_all(note.as_markdown().as_bytes()));
    }

    let child = Command::new(&editor)
                        .arg(path.to_str().unwrap())
                        .stdin(Stdio::inherit())
                        .stdout(Stdio::inherit())
                        .status()
                        .unwrap();

    let mut f = File::open(&path).unwrap();
    let mut new_note = parse_note(&mut f).unwrap();
    new_note.id = note.id;

    fs::remove_file(&path);
    Ok(new_note)
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

fn tmp_file_path() -> PathBuf {
    let mut rng = rand::thread_rng();
    let filename: String = rng.gen_ascii_chars().take(10).collect();
    let mut path: PathBuf = util::nt_dir().unwrap();
    path.push(filename);
    path
}

fn parse_note(fp: &mut File) -> Result<Note> {
    let mut reader = BufReader::new(fp);
    let mut lines = reader.lines();

    let name = lines.nth(0_usize).unwrap().unwrap();
    let mut content = String::new();

    for line in lines.skip(1_usize) {
        content.push_str(&line.unwrap());
        content.push_str("\n");
    }
    Ok(Note::new(None, &name, &content))
}

