use std::ops;
use std::io;
use crate::frame_buffer::*;


pub struct PpmEncoder<'a, W: 'a> {
    writer: &'a mut W,
}

impl<'a, W: io::Write + 'a> PpmEncoder<'a, W> {
    pub fn new(writer: &'a mut W) -> Self {
        Self { writer, }
    }

    pub fn encode<Storage>(&mut self, buffer: &FrameBuffer<Rgba<u8>, Storage>) -> io::Result<()> 
    where
        Storage: ops::Deref<Target = [u8]>,
    {
        write!(self.writer, "P3\n{} {}\n255\n", buffer.width(), buffer.height()).unwrap();
        for pixel in buffer.pixels() {
            writeln!(self.writer, "{} {} {}", pixel.r(), pixel.g(), pixel.b()).unwrap();    
        }

        Ok(())
    }
}

