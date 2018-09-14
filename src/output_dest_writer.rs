use std::io::Write;

mod file_out_writer;
mod std_out_writer;
pub use self::file_out_writer::FileOutWriter;
pub use self::std_out_writer::StdOutWriter;
use std::boxed::Box;

pub trait OutputDestWriter {
    fn get_writer(&mut self, path: &[&str]) -> Box<Write>;
}
