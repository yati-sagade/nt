use super::Note;
pub use sqlite3;

pub mod sqlite;

pub trait Store<'a> {
    type OutputIter: 'a + Iterator<Item=Note>;
    fn get(&mut self, id: isize) -> Option<Note>;
    fn put(&mut self, note: &mut Note) -> bool;
    fn del(&mut self, id: isize) -> bool;
    fn all(&'a mut self) -> Self::OutputIter;
}

pub trait Search<'a> {
    type OutputIter: 'a + Iterator<Item=Note>;
    fn search(&'a mut self, pattern: &str) -> Self::OutputIter;
}

