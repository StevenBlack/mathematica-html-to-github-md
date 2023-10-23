use regex::Regex;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Error: No input file passed.");
        return;
    }

    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Unable to read the file");

    // clean up excessive blank lines
    let contents = contents.replace("\n\n\n\n", "\n\n");
    let contents = contents.replace("\n\n\n", "\n\n");

    let mut lines: Vec<String> = contents.split("\n")
    .map(|s: &str| s.to_string())
    .collect();

    // First, throw out all the initial elements upto and inckuding
    // the <body> tag, and everything after the </body> tag, if
    // we have a body tag
    if lines.iter().any(|e| e.contains("<body")) {
        // up to <body...>
        let index = lines.iter().position(|r| r.starts_with("<body")).unwrap();
        lines.drain(..index + 1);

        // everything after </body...>
        let index = lines.iter().position(|r| r.starts_with("</body")).unwrap();
        lines.drain(index..);
    }

    // Back into a String
    let contents = lines.join("\n");

    // let classes = vec!["Title", "Section", "Subsection", "Subsubsection"];
    // let markdown = ["#", "##", "###", "####"];


    let regex = Regex::new(r###"<p class="Title">\n(.*)\n<\/p>"###).unwrap();
    let contents = regex.replace_all(contents.as_str(), "#$1").to_string();

    let regex = Regex::new(r###"<p class="Section">\n(.*)\n<\/p>"###).unwrap();
    let contents = regex.replace_all(contents.as_str(), "##$1").to_string();

    let regex = Regex::new(r###"<p class="Subsection">\n(.*)\n<\/p>"###).unwrap();
    let contents = regex.replace_all(contents.as_str(), "###$1").to_string();

    let regex = Regex::new(r###"<p class="Subsubsection">\n(.*)\n<\/p>"###).unwrap();
    let contents = regex.replace_all(contents.as_str(), "####$1").to_string();

    let regex = Regex::new(r###"<p class="Subtitle">\n (.*)\n<\/p>"###).unwrap();
    let contents = regex.replace_all(contents.as_str(), "**$1**").to_string();

    let regex = Regex::new(r###"<p class="ItemNumbered">\n(.*)\n<\/p>\n"###).unwrap();
    let contents = regex.replace_all(contents.as_str(), "1.$1").to_string();

    println!("{contents}");


}
