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


#[derive(Copy, Clone, Debug, PartialEq)]
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

#[derive(Copy, Clone, Debug, PartialEq)]
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

    fn parse_triangle(&mut self) -> Result<Triangle, TriLoaderError> {
        let vertex0 = self.parse_vertex()?;
        let vertex1 = self.parse_vertex()?;
        let vertex2 = self.parse_vertex()?;

        Ok(Triangle::new(vertex0, vertex1, vertex2))
    }

    fn parse_object(&mut self) -> Result<Vec<Triangle>, TriLoaderError> {
        let mut mesh: Vec<Triangle> = vec![];
        loop {
            match self.peek() {
                Some("\n") => {
                    self.skip_zero_or_more_newlines();
                }
                Some(_) => {
                    let triangle = self.parse_triangle()?;
                    mesh.push(triangle);
                }
                None => {
                    break;
                }
            }
        }

        Ok(mesh)
    }

    pub fn load(&mut self) -> Result<Vec<Triangle>, TriLoaderError> {
        self.parse_object()
    }
}


#[cfg(test)]
mod tests {
    use super::{
        TriLoader, 
        Triangle,
        Vertex,
    };


    #[test]
    fn test_loader_empty() {
        let data = String::from(r"");
        let expected = Ok(vec![]);
        let mut loader = TriLoader::new(&data);
        let result = loader.load();
        
        assert_eq!(result, expected);
    }


    #[test]
    fn test_lexer_one_line() {
        let data = String::from(r"
            -1.920570 0.000680 0.030130 -1.927750 0.102380 0.000170 -1.965040 0.013640 0.009820
        ");
        let expected = Ok(vec![
            Triangle::new(
                Vertex::new(-1.920570, 0.000680, 0.030130),
                Vertex::new(-1.927750, 0.102380, 0.000170),
                Vertex::new(-1.965040, 0.013640, 0.009820),
            )
        ]);
        let mut loader = TriLoader::new(&data);
        let result = loader.load();
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_lexer_multiple_lines() {
        let data = String::from(r"
            -1.920570 0.000680 0.030130 -1.927750 0.102380 0.000170 -1.965040 0.013640 0.009820     \
            -1.879360 0.093640 0.019450 -1.927750 0.102380 0.000170 -1.920570 0.000680 0.030130     \
            -1.879360 0.093640 0.019450 -1.889650 0.172390 -0.028960 -1.927750 0.102380 0.000170    \
            -1.838710 0.162240 -0.008080 -1.889650 0.172390 -0.028960 -1.879360 0.093640 0.019450   \
            -1.838710 0.162240 -0.008080 -1.844990 0.231520 -0.074300 -1.889650 0.172390 -0.028960  \
                                                                                                    \
                                                                                                    \
        ");
        let expected = Ok(vec![
            Triangle::new( 
                Vertex::new(-1.920570, 0.000680, 0.030130), 
                Vertex::new(-1.927750, 0.102380, 0.000170), 
                Vertex::new(-1.965040, 0.013640, 0.009820),
            ),
            Triangle::new(
                Vertex::new(-1.879360, 0.093640, 0.019450), 
                Vertex::new(-1.927750, 0.102380, 0.000170), 
                Vertex::new(-1.920570, 0.000680, 0.030130),
            ),
            Triangle::new(
                Vertex::new(-1.879360, 0.093640, 0.019450), 
                Vertex::new(-1.889650, 0.172390, -0.028960), 
                Vertex::new(-1.927750, 0.102380, 0.000170), 
            ),
            Triangle::new(
                Vertex::new(-1.838710, 0.162240, -0.008080),
                Vertex::new(-1.889650, 0.172390, -0.028960), 
                Vertex::new(-1.879360, 0.093640, 0.019450), 
            ),
            Triangle::new(
                Vertex::new(-1.838710, 0.162240, -0.008080), 
                Vertex::new(-1.844990, 0.231520, -0.074300), 
                Vertex::new(-1.889650, 0.172390, -0.028960), 
            ),
        ]);
        let mut loader = TriLoader::new(&data);
        let result = loader.load();
        
        assert_eq!(result, expected);
    }
}

