//
//  Checks the title of a webpage instead of HTML
//  alternative to TITLE
//

use crate::r#impl::write_result::write_result;
use reqwest::{Client, StatusCode};
use scraper::{Html, Selector};
use std::error::Error;
use std::process::Command;
use colored::*;
use super::websources::target_site;


pub async fn social_title_check(config: &target_site) -> Result<(), Box<dyn Error>> {
    let url = format!("{}{}", config.link, config.user_name);
    let request_url = url.to_string();

    if config.react_to_js {
        let output = Command::new("node")
            .arg("src/impl/fetch_title.js")
            .arg(url)
            .arg("TITLE")
            .arg(config.delay.to_string())
            .output()
            .expect("Failed to execute command");
        let title = String::from_utf8_lossy(&output.stdout).trim().to_string();

        if config.debug {
            println!("{}", title);
        }
        
        if (!config.reversed) {
            if title.to_lowercase().contains(&config.to_check.to_lowercase()) {
                println!("{}: {}", config.social_name, "FAILED".red());
            } else {
                println!("{}: {}", config.social_name, request_url.green());
                write_result(&format!("{}: {}", config.social_name, request_url));
            }
        } else {
            if title.to_lowercase().contains(&config.to_check.to_lowercase()) {
                println!("{}: {}", config.social_name, request_url.green());
                write_result(&format!("{}: {}", config.social_name, request_url));
            } else {
                println!("{}: {}", config.social_name, "FAILED".red());
            }
        }
    } else {
        if config.follow_redirs {
            let client = Client::builder()
                .redirect(reqwest::redirect::Policy::default())
                .build()?;
        }
        let response = reqwest::get(&request_url).await?;
        let body = response.text().await?;

        let document = Html::parse_document(&body);
        let title_selector = Selector::parse("title").unwrap();
        let title = document
            .select(&title_selector)
            .next()
            .and_then(|e| e.text().next())
            .unwrap_or("FYNDERERROR");

        if config.debug {
            println!("{}", title);
        }
        
        if (!config.reversed) {
            if title.contains(&config.to_check) {
                println!("{}: {}", config.social_name, "FAILED".red());
            } else {
                println!("{}: {}", config.social_name, request_url.green());
                write_result(&format!("{}: {}", config.social_name, request_url));
            }
        } else {
            if !title.contains(&config.to_check) {
                println!("{}: {}", config.social_name, "FAILED".red());
            } else {
                println!("{}: {}", config.social_name, request_url.green());
                write_result(&format!("{}: {}", config.social_name, request_url));
            }
        }
    }

    Ok(())
}
