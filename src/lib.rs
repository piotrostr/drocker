use log::info;
use std::fs::File;
use std::io::{BufReader, Error, Read};

/// Assumes that is called from the root of the project
pub fn read_in_template(fname: &str) -> Result<String, Error> {
    info!("Reading in template: {}", fname);
    let file = File::open(["./templates/", fname].join(""))?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    info!("Read in {} bytes", contents.len());
    return Ok(contents.to_string());
}
