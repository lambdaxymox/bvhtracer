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
    /*
    let mut triangles = vec![];
    for line in contents.split('\n') {
        let mut elements = [0_f32; 9];
        for (i, element_i) in line.split(' ').enumerate() {
            elements[i] = match element_i.parse::<f32>() {
                Ok(value) => value,
                Err(_) => return Err(TriLoaderError::new(
                    0,
                    ErrorKind::ExpectedFloat,
                    format!("Expected a floating point number but got `{}` instead.", element_i)
                )),
            };
        }

        let mut vertices = [Vector3::zero(); 3];
        for (i, vertex_i) in elements
            .chunks(3)
            .map(|chunk| Vector3::new(chunk[0], chunk[1], chunk[2]))
            .enumerate() 
        {
            vertices[i] = vertex_i;
        }

        let triangle = Triangle::new(vertices[0], vertices[1], vertices[2]);

        triangles.push(triangle);
    }

    Ok(triangles)
    */
}


pub fn load<P: AsRef<Path>>(path: P) -> Result<Vec<Triangle>, TriLoaderError> {
    let file = File::open(path).unwrap();
    let mut buf_reader = BufReader::new(file);
    from_reader(&mut buf_reader)
}