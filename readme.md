# 🤓 Mathematica HTML To GitHub Markdown

This is a personal project to convert some of my Mathematica "Save As" HTML
to Github Markdown. I want to do this so I can easily share Mathematica
notebooks on Github, specifically with an auto-generated readme that reflects
the current state of the notebook.

**Warning**: This code is not very robust. It's also not very general.

## Usage

These instructions presume you have a Mathematica notebook saved as HTML.

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Clone this repository.
3. `cd` into the repository.
3. Run `cargo run <path-to-html-file> > <path-to-markdown-file>`

Like this, for example:

```bash
cargo run ~/projects/notebook.htm > ~/projects/readme.md
```

Now your markdown file should be ready to go!
