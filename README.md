# 🤔 What is this?
It's a rust-based web scraping tool designed to extract and list significant web content.

# ✨ Features
- Extracts comment lines from HTML code snippets.
- Enumerates hyperlinks embedded within the webpage.
- Retrieves the content of the robots.txt file.


# ⚙️ How to install?

```
git clone https://github.com/0x1c1101/rws
cd rws/
cargo build
```

# 📖 How to use?

It's actually pretty simple.

```
❯ ./rws --help
Rust Web Scraper 1.0
heapoverfloww
A web scraper to enumerate web content written in Rust

USAGE:
    rws [FLAGS] --url <u>

FLAGS:
    -c, --comments    Get Comment Lines
    -h, --help        Prints help information
    -l, --links       Get links
    -r, --robots      Get robots.txt
    -V, --version     Prints version information

OPTIONS:
    -u, --url <u>    Specify a URL to send request
```
