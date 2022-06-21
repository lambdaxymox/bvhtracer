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
use super::lexer::*;


pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vertex {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, }
    }
}

pub struct Triangle {
    pub vertex0: Vertex,
    pub vertex1: Vertex,
    pub vertex2: Vertex,
}

impl Triangle {
    fn new(vertex0: Vertex, vertex1: Vertex, vertex2: Vertex) -> Self {
        Self { vertex0, vertex1, vertex2, }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ErrorKind {
    ExpectedFloat,
    LineHasTooFewVertices,
    LineHasTooManyVertices,
    EndOfFile,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TriLoaderError {
    /// The line number where the error occurred.
    pub line_number: usize,
    /// The kind of error that occurred.
    pub kind: ErrorKind,
    /// A message describing why the parse error was generated.
    pub message: String,
}

impl TriLoaderError {
    fn new(line_number: usize, kind: ErrorKind, message: String) -> Self {
        Self { line_number, kind, message, }
    }
}

pub struct TriLoader<'a> {
    line_number: usize,
    lexer: PeekableLexer<'a>,
}

impl<'a> TriLoader<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            line_number: 1,
            lexer: PeekableLexer::new(Lexer::new(input)),
        }
    }

    fn peek(&mut self) -> Option<&'a str> {
        self.lexer.peek()
    }

    fn error<T>(&self, kind: ErrorKind, message: String) -> Result<T, TriLoaderError> {
        Err(TriLoaderError::new(self.line_number, kind, message))
    }

    fn next(&mut self) -> Option<&'a str> {
        let token = self.lexer.next();
        if let Some(val) = token {
            if val == "\n" {
                self.line_number += 1;
            }
        }

        token
    }

    fn advance(&mut self) {
        self.next();
    }
    
    fn next_string(&mut self) -> Result<&'a str, TriLoaderError> {
        match self.next() {
            Some(st) => Ok(st),
            None => self.error(
                ErrorKind::EndOfFile,
                "Reached the end of the input in the process of getting the next token.".to_owned()
            )
        }
    }

    fn parse_f32(&mut self) -> Result<f32, TriLoaderError> {
        let st = self.next_string()?;
        match st.parse::<f32>() {
            Ok(val) => Ok(val),
            Err(_) => self.error(
                ErrorKind::ExpectedFloat,
                format!("Expected a floating point number but got `{}` instead.", st)
            ),
        }
    }

    fn parse_vertex(&mut self) -> Result<Vertex, TriLoaderError> {
        let x = self.parse_f32()?;
        let y = self.parse_f32()?;
        let z = self.parse_f32()?;

        Ok(Vertex::new(x, y, z))
    }

    fn skip_zero_or_more_newlines(&mut self) {
        while let Some("\n") = self.peek() {
            self.advance();
        }
    }

    /*
    fn skip_one_or_more_newlines(&mut self) -> Result<(), ParseError> {
        self.expect_tag("\n")?;
        self.skip_zero_or_more_newlines();
        Ok(())
    }
    */

    fn parse_triangle(&mut self) -> Result<Triangle, TriLoaderError> {
        let vertex0 = self.parse_vertex()?;
        let vertex1 = self.parse_vertex()?;
        let vertex2 = self.parse_vertex()?;

        Ok(Triangle::new(vertex0, vertex1, vertex2))
    }

    fn parse_object(&mut self) -> Result<Vec<Triangle>, TriLoaderError> {
        let mut triangles: Vec<Triangle> = vec![];
        loop {
            match self.peek() {
                Some("\n") => {
                    self.skip_zero_or_more_newlines();
                }
                Some(_) => {
                    let triangle = self.parse_triangle()?;
                    triangles.push(triangle);
                }
                None => {
                    break;
                }
            }
        }

        Ok(triangles)
    }

    pub fn load(&mut self) -> Result<Vec<Triangle>, TriLoaderError> {
        self.parse_object()
    }
}


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