#![allow(missing_docs)]
#![allow(dead_code)]
#![warn(unreachable_pub)]
#![deny(
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_variables,
    unused_assignments
)]
#![feature(nll)]
#![feature(try_trait)]
#![feature(box_patterns)]
#![feature(extern_in_paths)]
#![feature(pattern)]
#![feature(crate_visibility_modifier)]
#![feature(transpose_result)]

#[macro_use]
extern crate structopt;

mod args;
use extern::burgundy_generate;
use extern::failure::Error;

fn main() -> Result<(), Error> {
    let args = args::from_cmd_args();
    let api = burgundy_generate::load::Api::from_file(args.api_config_file)?;

    println!("Generating for {}", api.domain.name);

    let mut out_writer: Box<burgundy_generate::output_dest_writer::OutputDestWriter> =
        match args.directory {
            Some(dir) => Box::new(burgundy_generate::output_dest_writer::FileOutWriter::new(
                dir,
            )),
            None => Box::new(burgundy_generate::output_dest_writer::StdOutWriter::new()),
        };

    burgundy_generate::output::print(api, &mut *out_writer);

    Ok(())
}
