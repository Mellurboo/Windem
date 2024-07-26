use reqwest::{Client, StatusCode};
use scraper::{Html, Selector};
use std::error::Error;
use std::process::Command;
use std::str;
use std::env;
use colored::*;
use tokio::time::{sleep, Duration};

fn print_banner() {
    println!("{}","\
.____ ____ ____ ____ ____ ____ \n\
||F |||Y |||N |||D |||E |||M ||\n\
||__|||__|||__|||__|||__|||__||\n\
|/__\\|/__\\|/__\\|/__\\|/__\\|/__\\|\n\t@AvaLikesBread\n".green().bold());
}

async fn check_social_by_html(user_name: &str, link: &str, social_name: &str, to_check: &str, follow_redirs: bool, react_to_js: bool, debug: bool) -> Result<(), Box<dyn Error>> {
    let url = format!("{}{}", link, user_name);
    let request_url = url.to_string();

    if react_to_js {
        let output = Command::new("node")
            .arg("fetch_title.js")
            .arg(url)
            .arg("HTML")
            .output()
            .expect("\nThere was an error loading/running the fetch_title.js script\nExiting program...");
        let body = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if debug { println!("{body}");}
        if body.contains(to_check) {
            println!("{}: {}", social_name, "FAILED".red());
        } else {
            println!("{}: {}", social_name, request_url.green());
        }
    } else {
        if follow_redirs {
            let client = Client::builder().redirect(reqwest::redirect::Policy::default()).build()?;
        }

        let response = reqwest::get(&request_url).await?;
        let body = response.text().await?;
        if debug { println!("{body}"); }
        if body.contains(to_check) {
            println!("{}: {}", social_name, "FAILED".red());
        } else {
            println!("{}: {}", social_name, request_url.green());
        }
    }

    Ok(())
}

async fn check_social_by_title(user_name: &str, link: &str, social_name: &str, to_check: &str, follow_redirs: bool, react_to_js: bool, debug: bool) -> Result<(), Box<dyn Error>> {
    let url = format!("{}{}", link, user_name);
    let request_url = url.to_string();
    if react_to_js {
        let output = Command::new("node")
            .arg("fetch_title.js")
            .arg(url)
            .arg("TITLE")
            .output()
            .expect("Failed to execute command");
        let title = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if debug { println!("{title}"); }
        if title.contains(to_check) {
            println!("{}: {}", social_name, "FAILED".red());
        } else {
            println!("{}: {}",social_name, request_url.green());
        }
    } else {
        if follow_redirs {
        let client = Client::builder().redirect(reqwest::redirect::Policy::default()).build()?;
    }
        let response = reqwest::get(&request_url).await?;
        let body = response.text().await?;
    
        let document = Html::parse_document(&body);
        let title_selector = Selector::parse("title").unwrap();
        let title = document
            .select(&title_selector)
            .next()
            .and_then(|e| e.text().next())
            .unwrap_or("F   YNDERERROR");
        if debug { println!("{title}"); }
        if title.contains(to_check) || title == "FYNDERERROR" {
            println!("{}: {}", social_name, "FAILED".red());
        } else {
            println!("{}: {}",social_name, request_url.green());
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("{}\nUsage: {} {}\n{}\n\t{}: debug", "ARGUMENT ERROR!".red().bold(), "fyndem".green().bold(), "<username>".yellow(), "Arguments".red().bold(), "-d".yellow());
        std::process::exit(1);
    }

    let debug = args.contains(&"-d".to_string());

    print_banner();
    check_social_by_title(&format!("@{}", &args[1]), "https://www.youtube.com/", "YouTube", "404 Not Found", false, false, debug).await;
    check_social_by_title(&format!("@{}", &args[1]), "https://www.tiktok.com/", "TikTok", "Couldn’t find this account. Visit TikTok to discover more trending creators, hashtags, and sounds.", true, true, debug).await;
    check_social_by_title(&args[1], "https://www.flickr.com/photos/", "Flickr", "Oops! | Flickr", false, true, debug).await;
    check_social_by_title(&args[1], "https://github.com/", "Github", "Page not found", false, true, debug).await;
    check_social_by_html(&format!("{}.bsky.social", &args[1]), "https://bsky.app/profile/", "Bluesky", "Unable to resolve handle", false, true, debug).await;

    // Sadly, Fynder cannot confirm some webpages... So, we make it easier for the user to do so by generating the links for them!

    println!("{}","\nFyndem can't conirm some webpages due to technical issues. Here are the links to check for yourself:".yellow().bold());
    println!("X: https://x.com/{}/",&args[1].yellow());
    println!("Reddit: https://www.reddit.com/user/{}/", &args[1].yellow());
}

