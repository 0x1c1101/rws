use reqwest;
use std::env;
use std::time::Instant;

static COLOR_RED: &str = "\x1b[31m";
static COLOR_GREEN: &str = "\x1b[32m";
static COLOR_YELLOW: &str = "\x1b[33m";
static COLOR_BLUE: &str = "\x1b[34m";
static COLOR_MAGENTA: &str = "\x1b[35m";
static COLOR_CYAN: &str = "\x1b[36m";
static COLOR_RESET: &str = "\x1b[0m";


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

fn is_valid_url(url: &str) -> bool {
    if !url.contains(".") {
        return false;
    }

    true
    
}

fn get_root_url(mut newurl: String) -> String {
    if !newurl.ends_with("/") {
        newurl.push('/');
    }

    let mut begin : usize = 0;
    let mut end : usize = 0;
    let mut i : usize = 0;
    for c in newurl.chars() {
        if c == '.' {
            begin = i;
        }
        else if c == '/' && begin != 0 && i > begin {
            end = i + 1;
            break;
        }

        i += 1;
    }


    newurl[0..end].to_string()
}


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
        links.dedup();
    }
    if get_comments {
        comments.dedup();
    }
}

fn usage(arg: &String){
    println!("{COLOR_RED}Usage: {} <URL> <OPTS>", arg);
    println!("Current options:");
    println!("r: Read robots.txt");
    println!("c: List comment lines in the page");
    println!("l: List hyperlinks in the page");
    println!("A: Enable all options");
    println!("\n{COLOR_YELLOW}Example: {} https://example.com rlc{COLOR_RESET}", arg);
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        usage(&args[0]);
        return;
    }

    let start_time = Instant::now();


    let url = &args[1];
    let opts = &args[2];

    if !is_valid_url(url){
        println!("{COLOR_RED}Please provide a valid URL.{COLOR_RESET}");
        return;
    }

    if !opts.contains(['A', 'c', 'l', 'r']){
        usage(&args[0]);
        return;
    }

    let banner = format!(
        "{COLOR_CYAN}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{COLOR_RESET}",
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
    
    println!("{}",banner);
    

    match get_body(&url).await {
        Ok(body) => {
            
            if opts.contains(['A', 'r']) {
                
                let newurl = get_root_url(url.clone()) + "robots.txt";
                match get_body(&newurl).await {
                    Ok(robots) => {
                        println!("{COLOR_BLUE}\nrobots.txt:\n\n{}", robots);
                    },
                    Err(err) => eprintln!("Failed to get robots.txt: {:?}", err),
                }
                
                

            }

            let mut comments : Vec<String> = vec![];
            let mut links : Vec<String> = vec![];
            scrape(&body, &mut comments, &mut links, opts.contains(['A', 'l']), opts.contains(['A', 'c']));
            if opts.contains(['A', 'c']){

                println!("\n{COLOR_GREEN}Comments:\n");
                for comment in comments {
                    if comment.len() < 2 { 
                        continue;
                    }
                    println!("{}\n---", comment);
                }

            }
            if opts.contains(['A', 'l']) {
                println!("\n{COLOR_MAGENTA}Hyperlinks:\n");
                for link in links {
                    if link.len() < 2 {
                        continue;
                    }
                    println!("{}", link);
                }

            }
            
            
        },
        Err(e) => eprintln!("Failed to get HTML content: {:?}", e),
    }

    let elapsed_time = Instant::now() - start_time;
    println!("\n{COLOR_YELLOW}It took {} seconds!", elapsed_time.as_secs_f64());
}