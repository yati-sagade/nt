use super::{Search,Store};
use std::collections::HashMap;
use sqlite3::types::{BindArg, RowMap, ResultCode};
use super::super::Note;
use super::sqlite3;

static INIT_DB_SQL: &'static str = "CREATE TABLE IF NOT EXISTS notes(
                                        id INTEGER PRIMARY KEY,
                                        name TEXT,
                                        content TEXT,
                                        created DATETIME DEFAULT CURRENT_TIMESTAMP,
                                        last_updated DATETIME DEFAULT CURRENT_TIMESTAMP
                                    );";

static ALL_NOTES_SQL: &'static str = "SELECT id, name, content, created, last_updated FROM notes ORDER BY id;";

static NOTE_INSERT_SQL: &'static str = "INSERT INTO notes(name, content) VALUES(?, ?);";

static NOTE_UPDATE_SQL: &'static str = "UPDATE TABLE notes SET name=?, content=? WHERE id=?;";

static NOTE_BY_ID_SQL: &'static str = "SELECT id, name, content, created, last_updated FROM notes WHERE id=?;";

static NOTE_DELETE_SQL: &'static str = "DELETE FROM notes WHERE id = ?;";

static NOTE_SEARCH_SQL: &'static str = "SELECT id, name, content, created, last_updated FROM notes
                                         WHERE name LIKE ? OR content LIKE ? ORDER BY id;";

pub struct SQLiteStoreIterator<'a> {
    cursor: sqlite3::Cursor<'a>,
}

impl<'a> Iterator for SQLiteStoreIterator<'a> {
    type Item = Note;
    fn next(&mut self) -> Option<Note> {
        next_note_from_cursor(&mut self.cursor)
    }
}

pub struct SQLiteStore {
    db: sqlite3::Database
}

impl SQLiteStore {
    pub fn open(path: &str) -> SQLiteStore {
        let mut db = sqlite3::open(path).unwrap();
        let _ = db.exec(INIT_DB_SQL).unwrap(); 
        SQLiteStore {db: db}
    }

    fn del_note(&mut self, note: &mut Note) -> bool {
        match note.id {
            Some(id) => {
                note.id = None;
                self.del(id)
            },
            None => false
        }
    }
}

impl<'a> Search<'a> for SQLiteStore {
    type OutputIter = SQLiteStoreIterator<'a>;

    fn search(&'a mut self, pattern: &str) -> SQLiteStoreIterator<'a> {
        let mut cursor = self.db.prepare(NOTE_SEARCH_SQL, &None).unwrap();
        let mut pattern_string = String::new();
        pattern_string.push_str("%");
        pattern_string.push_str(pattern);
        pattern_string.push_str("%");
        let args: &[BindArg] = &[BindArg::Text(pattern_string.clone()),
                                 BindArg::Text(pattern_string)];
        match cursor.bind_params(args) {
            ResultCode::SQLITE_OK => { },
            x => panic!(format!("The error code from SQLite3 was {:?} ", x))
        };
        SQLiteStoreIterator::<'a>{cursor: cursor}
    }
}

impl<'a> Store<'a> for SQLiteStore {
    type OutputIter = SQLiteStoreIterator<'a>;

    fn get(&mut self, id: isize) -> Option<Note> {
        let mut cursor = self.db.prepare(NOTE_BY_ID_SQL, &None).unwrap();
        let args: &[BindArg] = &[BindArg::Integer(id)];
        match cursor.bind_params(args) {
            ResultCode::SQLITE_OK => { },
            _ => { return None; }
        };
        next_note_from_cursor(&mut cursor)
    }

    fn put(&mut self, note: &mut Note) -> bool {
        match note.id {
            Some(id) => {
                let mut cursor = self.db.prepare(NOTE_UPDATE_SQL, &None).unwrap();
                let args: &[BindArg] = &[BindArg::Text(note.name.clone()),
                                         BindArg::Text(note.content.clone()),
                                         BindArg::Integer(id)];
                match cursor.bind_params(args) {
                    ResultCode::SQLITE_OK =>  { },
                    _ => { return false; }
                };
                match cursor.step() {
                    ResultCode::SQLITE_DONE => true,
                    _ => false
                }
            },
            None => {
                let mut cursor = self.db.prepare(NOTE_INSERT_SQL, &None).unwrap();
                let args: &[BindArg] = &[BindArg::Text(note.name.clone()),
                                         BindArg::Text(note.content.clone())];
                match cursor.bind_params(args) {
                    ResultCode::SQLITE_OK => { },
                    _ => { return false; }
                }
                match cursor.step() {
                    ResultCode::SQLITE_DONE => {
                        note.id = Some(self.db.get_last_insert_rowid() as isize);
                        true
                    }
                    _ => false
                }
            }
        }
    }
    
    fn del(&mut self, id: isize) -> bool {
        let mut cursor = self.db.prepare(NOTE_DELETE_SQL, &None).unwrap();
        let args: &[BindArg] = &[BindArg::Integer(id)];
        match cursor.bind_params(args) {
            ResultCode::SQLITE_OK => { },
            _ => { return false; }
        }
        match cursor.step() {
            ResultCode::SQLITE_DONE => {
                true
            }
            _ => false
        }
    }

    fn all(&'a mut self) -> SQLiteStoreIterator<'a> {
        let cursor = self.db.prepare(ALL_NOTES_SQL, &None).unwrap();
        SQLiteStoreIterator::<'a>{cursor: cursor}
    }

}

struct RowMapGetter {
    row_map: RowMap
}

impl RowMapGetter {
    fn new(row_map: &RowMap) -> Self {
        let mut r = HashMap::new();
        r.clone_from(row_map);
        RowMapGetter { row_map: r }
    }

    fn get_int(&self, key: &str) -> Option<isize> {
        match self.row_map.get(key) {
            Some(&BindArg::Integer(val)) => Some(val),
            Some(&BindArg::Integer64(val)) => Some(val as isize),
            _ => None,
        }
    }

    fn get_text(&self, key: &str) -> Option<String> {
        match self.row_map.get(key) {
            Some(&BindArg::Text(ref val)) => Some(val.clone()),
            _ => None,
        }
    }

    fn get_blob(&self, key: &str) -> Option<Vec<u8>> {
        match self.row_map.get(key) {
            Some(&BindArg::Blob(ref blob)) => Some(blob.clone()),
            _ => None,
        }
    }
}

fn note_from_rowmap(row_map: &RowMap) -> Option<Note> {
    let rmg = RowMapGetter::new(row_map);
    let id = rmg.get_int("id").unwrap();
    let name = rmg.get_text("name").unwrap();
    let content = rmg.get_text("content").unwrap();
    Some(Note{ id: Some(id), name: name, content: content })
}

fn next_note_from_cursor(cursor: &mut sqlite3::Cursor) -> Option<Note> {
    match cursor.step_row() {
        Ok(None) | Err(_) => None,
        Ok(Some(ref row_map)) => {
            note_from_rowmap(row_map)
        }
    }
}

