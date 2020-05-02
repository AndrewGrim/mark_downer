use std::io;
use std::env;

use markdowner;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("ERROR: The program needs three arguments!");
        println!("\t$ md <input> <output> <css>");
    } else {
        let input = args.get(1).unwrap().as_str();
        let output = args.get(2).unwrap().as_str();
        let css = args.get(2).unwrap().as_str();
        markdowner::markdown_to_html(input, output, css)?;
    }

    Ok(())
}