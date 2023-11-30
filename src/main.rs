use reqwest;
use clap::{App, Arg};
use url::Url;
use std::process;
use colored::Colorize;
use std::collections::HashSet;
use core::hash::Hash;

// Operations Begin

fn remove_duplicates<T: Eq + Hash + Clone>(vec: &mut Vec<T>) {
    let set: HashSet<_> = vec.drain(..).collect();
    vec.extend(set);
}

async fn get_body(url: &str) -> Result<String, reqwest::Error> {
    let ua: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36";

    let body = reqwest::Client::new()
        .get(url)
        .header(reqwest::header::USER_AGENT, ua)
        .send()
        .await?
        .text()
        .await?;

    Ok(body)
}

fn is_valid_url(input_url: &str) -> bool {
    match Url::parse(input_url) {
        Ok(_) => true,
        Err(_) => false,
    }
}

// Operations End

fn scrape (content: &str, comments: &mut Vec<String>, links: &mut Vec<String>, get_links: bool, get_comments: bool)
{
    let mut in_comment = false;
    let mut in_href = false;
    let mut chars = String::new();
    let mut chars2 = String::new();
    for c in content.chars() {

        // Links
        if get_links {
            if c != ' ' {
                chars2.push(c);
            }
            if in_href && chars2.ends_with("\"") {
                in_href = false;
                chars2.truncate(chars2.len() - 1); // Removing "
                links.push(chars2.clone()); // Adding it to the vector
                chars2.clear();
            }
            else if !in_href && chars2.ends_with("href=\"") {
                in_href = true;
                chars2.clear();
            }

        }

        if get_comments {
            chars.push(c);
            if chars.ends_with("-->") {
                in_comment = false;
                chars.truncate(chars.len() - 3); // Removing "-->"
                comments.push(chars.clone()); // Adding it to the vector
                chars.clear();
            }
            else if !in_comment && chars.ends_with("<!--") {
                in_comment = true;
                chars.clear();
            }
        }
    }

    if get_links { 
        remove_duplicates(links);
    }
    if get_comments { 
        remove_duplicates(comments);
    }
}


#[tokio::main]
async fn main() {
    
    
    let matches = App::new("Rust Web Scraper")
        .version("1.0")
        .author("heapoverfloww")
        .about("A web scraper to enumerate web content written in Rust")
        .arg(
            Arg::with_name("l")
                .short("l")
                .long("links")
                .help("Get links"),
        )
        .arg(
            Arg::with_name("c")
                .short("c")
                .long("comments")
                .help("Get Comment Lines"),
        )
        .arg(
            Arg::with_name("r")
                .short("r")
                .long("robots")
                .help("Get robots.txt"),
        )
        .arg(
            Arg::with_name("u")
                .short("u")
                .long("url")
                .takes_value(true)
                .help("Specify a URL to send request")
                .required(true)
        )
        .get_matches();

    let banner = format!(
        "\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
        r#":::::::::  :::       :::  ::::::::  "#,
        r#":+:    :+: :+:       :+: :+:    :+:"#,
        r#"+:+    +:+ +:+       +:+ +:+        "#,
        r#"+#++:++#:  +#+  +:+  +#+ +#++:++#++ "#,
        r#"+#+    +#+ +#+ +#+#+ +#+        +#+ "#,
        r#"#+#    #+#  #+#+# #+#+#  #+#    #+# "#,
        r#"###    ###   ###   ###    ########  "#,
        r#"------------------------------------"#,
        r#"Rust Web Scraper      @heapoverfloww"#
    );
    
    println!("{}",banner.truecolor(0, 255, 136));

    let mut url : &str = "";
    if let Some(val) = matches.value_of("u"){
        if is_valid_url(val){
            url = val;
        }
        else {
            println!("{} is not a valid URL", val);
            process::exit(1);
        }
    }

    
        
    match get_body(url).await {
        Ok(body) => {
            
            if matches.is_present("r") {
                let url_parsed = Url::parse(url).expect("Failed to parse URL");
                let root_url = format!(
                    "{}://{}{}/robots.txt",
                    url_parsed.scheme(),
                    url_parsed.host_str().unwrap(),
                    if let Some(port) = url_parsed.port() {
                        format!(":{}", port)
                    } else {
                        "".to_string()
                    }
                );
            
                match get_body(&root_url).await {
                    Ok(robots) => {
                        println!("\n{}\n", "robots.txt:".red());
                        println!("{}", robots.green());
                    },
                    Err(err) => eprintln!("Failed to get robots.txt: {:?}", err),
                }
            }

            {
                let mut comments : Vec<String> = vec![];
                let mut links : Vec<String> = vec![];
                scrape(&body, &mut comments, &mut links, matches.is_present("l"), matches.is_present("c"));
                if matches.is_present("c"){

                    println!("\n{}\n", "Comments:".red());
                    for comment in comments {
                        if comment.len() < 2 { 
                            continue;
                        }
                        println!("<!--\n{}\n-->", comment.green());
                    }

                }
                if matches.is_present("l") {
                    println!("\n{}\n", "Hyperlinks:".red());
                    for link in links {
                        if link.len() < 2 {
                            continue;
                        }
                        println!("{}", link.green());
                    }

                }
            }
            

        },
        Err(e) => eprintln!("Failed to get HTML content: {:?}", e),
    }


}
