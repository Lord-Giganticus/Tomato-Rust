use std::env;

mod exports;

fn main() {
    let envargs: Vec<String> = env::args().collect();
    let args = &envargs[1..];
    if args.len() > 1 {
        if exports::bmgtoxml(&args[0], &args[1]) {
            println!();
            println!("Conversion succeeded.");
        }
    } else if args.len() == 1 {
        if exports::xmltobmg(&args[0], false) {
            println!();
            println!("Conversion succeeded.");
        }
    } else {
        panic!("No args!");
    }
}