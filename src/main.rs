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
        .expect("Unable to read the input file");

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
    let mut contents = lines.join("\n");

    // Transform the headings
    let headings = vec![
        ("Title", "#"),
        ("Section", "##"),
        ("Subsection", "###"),
        ("Subsubsection", "####"),
    ];
    for heading in headings {
        // let s3: String = format!("{}{}", s1, s2);
        let foo2 = format!(r###"<p class="{}">\n(.*)\n<\/p>"###, heading.0);
        let regex = Regex::new(foo2.as_str()).unwrap();
        // contents = regex.replace_all(contents.as_str(), "#$1").to_string();
        contents = regex.replace_all(contents.as_str(), format!(r"{}$1", heading.1)).to_string();
    }

    // Transform Subtitle classes
    let regex = Regex::new(r###"<p class="Subtitle">\n (.*)\n<\/p>"###).unwrap();
    let contents = regex.replace_all(contents.as_str(), "**$1**").to_string();

    // Transform ordered lists
    let regex = Regex::new(r###"<p class="ItemNumbered">\n(.*)\n<\/p>\n"###).unwrap();
    let contents = regex.replace_all(contents.as_str(), "1.$1\n").to_string();

    // Transform italic and bold text
    let regex = Regex::new(r###"<span style='font-style: italic;font-weight: bold;'>(.*)<\/span>"###).unwrap();
    let contents = regex.replace_all(contents.as_str(), "***$1***").to_string();

    // Transform bold text
    let regex = Regex::new(r###"<span style='font-weight: bold;'>(.*)<\/span>"###).unwrap();
    let contents = regex.replace_all(contents.as_str(), "**$1**").to_string();

    // Transform italic text
    // need to make this ungreedy
    // let regex = Regex::new(r###"<span style='font-style: italic;'>(.*)<\/span>"###).unwrap();
    // let contents = regex.replace_all(contents.as_str(), "*$1*").to_string();

    println!("{contents}");

}
