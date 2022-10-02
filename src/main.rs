use rocker::read_in_template;

fn main() {
    env_logger::init();

    let template = read_in_template("rust.tpl").ok().unwrap();
    println!("{}", template);
}
