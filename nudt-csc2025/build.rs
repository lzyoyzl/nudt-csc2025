fn main() {
    lalrpop::Configuration::new()
        .process_file("src/frontend/sys_y/sysy.lalrpop")
        .unwrap();
}
