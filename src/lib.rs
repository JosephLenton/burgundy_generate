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
extern crate serde_derive;

pub mod load;
pub mod output;
pub mod output_dest_writer;
