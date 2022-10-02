use rocker::read_in_template;

#[test]
fn it_works() {
    let fname = "rust.tpl";
    match read_in_template(fname) {
        Ok(_) => assert!(true),
        Err(e) => {
            println!("Error: {}\n", e);
            assert!(false);
        }
    }
}
