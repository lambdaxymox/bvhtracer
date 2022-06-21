mod lexer;
mod loader;


pub use crate::loader::{
    Vertex,
    Triangle,
    TriLoaderError,
};

use crate::loader::{
    TriLoader,
};

use std::path::{
    Path,
};
use std::fs::{
    File,
};
use std::io;
use std::io::{
    BufReader,
};



pub fn from_reader<R: io::Read + io::Seek>(reader: &mut R) -> Result<Vec<Triangle>, TriLoaderError> {
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();
    let mut loader = TriLoader::new(&contents);
    loader.load()
}


pub fn load<P: AsRef<Path>>(path: P) -> Result<Vec<Triangle>, TriLoaderError> {
    let file = File::open(path).unwrap();
    let mut buf_reader = BufReader::new(file);
    from_reader(&mut buf_reader)
}

