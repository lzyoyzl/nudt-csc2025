fn main() {
    lalrpop::Configuration::new()
        .process_file("src/frontend/sys_y/grammar.lalrpop")
        .unwrap();
}
