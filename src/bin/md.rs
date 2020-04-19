use std::io;
use std::env;

use markdowner;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let input = args.get(1).unwrap().as_str();
    let output = args.get(2).unwrap().as_str();

    markdowner::markdown_to_html(input, output)?;

    Ok(())
}