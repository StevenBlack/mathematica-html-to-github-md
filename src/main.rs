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


    // Here we audit Mathematica's HTML document for headings used, assigning one of GitHub's
    // markup headings to each one used.

    // Here's Mathematica's heading hierarchy
    let mathematica_headings = vec![
        "Title",
        "Chapter",
        "Section",
        "Subsection",
        "Subsubsection",
    ];

    // GitHub supports 6-levels of heading ("#" to "######")
    let mut gh_level = 1;

    let mut doc_headings: Vec<(String, String)> = vec![];

    for m_heading in mathematica_headings {
        let test = r##"<p class=""##.to_owned() + m_heading + r##"">"##;
        if contents.contains(test.as_str()) {
            doc_headings.push((m_heading.to_string(), "#".repeat(gh_level).to_string()));
            gh_level = gh_level + 1;
        }
    }

    for heading in doc_headings {
        // let s3: String = format!("{}{}", s1, s2);
        let foo = format!(r###"<p class="{}">\n(.*)\n<\/p>"###, heading.0);
        let regex = Regex::new(foo.as_str()).unwrap();
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
    let regex = Regex::new(r###"<span style='font-weight: bold;'>(.*?)<\/span>"###).unwrap();
    let contents = regex.replace_all(contents.as_str(), "**$1**").to_string();

    let regex = Regex::new(r###"<span style='font-style: italic;'>(.*?)<\/span>"###).unwrap();
    let mut contents = regex.replace_all(contents.as_str(), "*$1*").to_string();

    // Remove the spurrious <p class="xxx">
    let p_classes = vec!["Author", "Text"];
    for p_class in p_classes {
        let foo = format!(r###"<p class="{}">\n(.*)\n<\/p>"###, p_class);
        let regex = Regex::new(foo.as_str()).unwrap();
        contents = regex.replace_all(contents.as_str(), "$1").trim().to_string();
    }

    println!("{contents}");

}
