use rocker::{dockerfile_exists, read_in_template, write_dockerfile};

fn main() {
    env_logger::init();

    if dockerfile_exists() {
        println!("Dockerfile already exists");
        return;
    }

    let template = read_in_template("rust.tpl").ok().unwrap();

    match write_dockerfile(template) {
        Ok(_) => println!("Wrote Dockerfile"),
        Err(e) => println!("Error writing Dockerfile: {}", e),
    }
}
