//! Binary that can convert a markdown file to an html file.
//! # Example
//! `md <input> <output> <css>`
//!
//! `input` - The markdown file to be translated to html.
//!
//! `output` - The path to where the translated file will be saved.
//!
//! `css` - The path to the css file. This argument is optional. 
//! For making your own style take a look at the included css file.

use std::io;
use std::env;

use markdowner;

/// Parses args and calls the lib to generate html.
fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 {
        // Convert Markdown to HTML without css.
        let input = args.get(1).unwrap().as_str();
        let output = args.get(2).unwrap().as_str();
        markdowner::markdown_to_html(input, output, "")?;
    } else if args.len() == 4 {
        // Convert Markdown to HTML and embed CSS.
        let input = args.get(1).unwrap().as_str();
        let output = args.get(2).unwrap().as_str();
        let css = args.get(3).unwrap().as_str();
        markdowner::markdown_to_html(input, output, css)?;
    }  else {
        println!("ERROR: Wrong number of arguments!");
        println!("\t$ md <input> <output> <css>");
        println!("NOTE: The <css> argument is optional.");
    }

    Ok(())
}