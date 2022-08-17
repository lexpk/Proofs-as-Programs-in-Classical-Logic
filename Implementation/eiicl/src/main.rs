mod translation;

use std::env;

fn main() {
    let mut args = env::args();
    let problem : translation::Problem;
    match args.len() {
        1 => {
            panic!("Please provide input file.")
        },
        _ => {
            problem = translation::Problem::new(args.nth(1).unwrap(), args.nth(0));
        },
    }
    for annotated in problem.translate().to_tptp() {
        println!("{}", annotated);
    }
}
