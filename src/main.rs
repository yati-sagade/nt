extern crate nt;
extern crate time;
extern crate getopts;

use std::io::Read;
use getopts::{Options,Matches};

use nt::core::Note;
use nt::core::persistence::Store;
use nt::core::editor::edit_note;
use nt::core::persistence::Search;
use nt::core::persistence::sqlite::SQLiteStore;


fn main() {

    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();
    let opts = get_options();
    let matches: Matches = opts.parse(&args[1..]).unwrap();
    let mut store = SQLiteStore::open("notes.db");

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.opt_present("l") {
        let blue = "\x1b[94m";
        let nc = "\x1b[0m";
        let msg = format!("{}Listing all{}", blue, nc);
        println!("{}", msg);
        for note in store.all() {
            println!("{}{}. {}{}", blue, note.id.unwrap(), &note.name, nc);
        }
        println!("");
        return;
    }

    if matches.opt_present("n") {
        println!("Enter note body followed by EOF (Ctrl + D on UNIX): ");
        let note_name = matches.opt_str("n").unwrap();
        let stdin = std::io::stdin();
        let mut content = String::new();
        stdin.lock().read_to_string(&mut content).unwrap();
        let mut note = Note::new(None, &note_name, &content);
        store.put(&mut note);
        return;
    }

    if matches.opt_present("d") {
        let note_id_str = matches.opt_str("d").unwrap();
        let note_id = match note_id_str.parse::<isize>() {
            Ok(id) => id,
            _ => {
                println!("Invalid id {} ", note_id_str);
                return;
            }
        };

        let note = match store.get(note_id) {
            Some(note) => note,
            None => {
                println!("No such note with id {}.", note_id);
                return;
            }
        };

        println!("{}. {}", note.id.unwrap(), note.name);
        println!("");
        println!("{}", note.content);

        return;
    }

    if matches.opt_present("x") {
        let note_id_str = matches.opt_str("x").unwrap();
        let note_id = match note_id_str.parse::<isize>() {
            Ok(id) => id,
            _ => {
                println!("Invalid id {}", note_id_str);
                return;
            }
        };
        if !store.del(note_id) {
            println!("No such note with id {} ", note_id);
        } else {
            println!("Deleted note #{}", note_id);
        }
        return;
    }

    if matches.opt_present("s") {
        let pattern = matches.opt_str("s").unwrap();
        for note in store.search(&pattern) {
            println!("{}\n\n", note);
        }
        return;
    }
    
    if matches.opt_present("e") {
        let note_id_str = matches.opt_str("e").unwrap();
        let note_id = match note_id_str.parse::<isize>() {
            Ok(id) => id,
            _ => {
                println!("Invalid id {}", note_id_str);
                return;
            }
        };
        let note = store.get(note_id).unwrap();
        edit_note(&note);
        return;
    }

    let timespec = time::get_time();
    let content: &str = &format!("{}", timespec.sec);
    let mut note = Note::new(None, "Curent time", content);
    store.put(&mut note);
    println!("Now listing all:\n");
    for note in store.all() {
        println!("{}", note);
    }
}

fn get_options() -> Options {
    let mut opts = Options::new();
    opts.optopt("n", "new", "start a note with title", "TITLE");
    opts.optopt("d", "display", "show note contents", "NOTE_ID");
    opts.optopt("e", "edit", "edit a note", "NOTE_ID");
    opts.optopt("x", "delete", "delete a note", "NOTE_ID");
    opts.optopt("s", "search", "search for a textual pattern", "PATTERN");
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("l", "list", "list all notes");
    opts
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}



