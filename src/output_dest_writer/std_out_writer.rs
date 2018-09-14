use crate::output_dest_writer::OutputDestWriter;
use std::boxed::Box;
use std::io::stdout;
use std::io::Write;

pub struct StdOutWriter;

impl StdOutWriter {
    pub fn new() -> Self {
        StdOutWriter
    }
}

impl OutputDestWriter for StdOutWriter {
    fn get_writer(&mut self, _path: &[&str]) -> Box<Write> {
        Box::new(stdout())
    }
}
