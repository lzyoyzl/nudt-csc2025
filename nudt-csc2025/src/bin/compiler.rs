use std::env::args;
use std::fs;
use lalrpop_util::lalrpop_mod;
lalrpop_mod!(#[allow(clippy::all)] pub sysy, "/frontend/sys_y/sysy.rs");

fn main()
{
    //dealing with the args
    let mut args=args();
    args.next(); // skip the program name
    let input=args.next().unwrap();
    args.next(); // skip -o
    let output=args.next().unwrap();

    //reading input file
    let contents = fs::read_to_string(input)
        .expect("Should have been able to read the file");

    let ast = sysy::CompUnitParser::new().parse(&contents).unwrap();
    // println!("{:#?}", ast);
    // println!("hello world1");
}