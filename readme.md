# 🤓 Mathematica HTML To GitHub Markdown

This is a personal project to convert Mathematica HTML files to Github Markdown.  
This code is not very robust. It's also not very general.

Motivation: I wanted a nice way to convert my Mathematica notebooks, when saved as HTML, to Github Markdown.
I wanted to do this so I could easily share my Mathematica notebooks on Github with an auto-generated readme
that reflects the current state of the notebook.

Sample call:

```bash
cargo run /Users/steve/Dropbox/btc/bitcoin-energy-estimates/bitcoin-energy-estimates.htm > /Users/steve/Dropbox/btc/bitcoin-energy-estimates/readme.md
```
