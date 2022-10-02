use log::info;
use std::fs::File;
use std::io::{BufReader, BufWriter, Error, Read, Write};
use std::path::Path;

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

pub fn write_dockerfile(contents: String) -> Result<(), Error> {
    info!("Writing Dockerfile");
    let file = File::create("Dockerfile")?;
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write_all(contents.as_bytes())?;
    Ok(())
}

pub fn dockerfile_exists() -> bool {
    info!("Checking if Dockerfile exists");
    Path::new("Dockerfile").exists()
}

pub fn add_args() {
    info!("Adding args");
}
