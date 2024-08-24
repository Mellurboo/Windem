//
//  For sites that require HTML Checking over the TITLE checking
//

use reqwest::{Client, StatusCode};
use std::error::Error;
use std::process::Command;
use colored::*;
use tokio::time::{sleep, Duration};
use super::websources::target_site;


pub async fn social_html_check(config: &target_site) -> Result<(), Box<dyn Error>> {
    let url = format!("{}{}", config.link, config.user_name);
    let request_url = url.to_string();

    if config.react_to_js {
        let output = Command::new("node")
            .arg("src/impl/fetch_title.js")
            .arg(url)
            .arg("HTML")
            .output()
            .expect("\nThere was an error loading/running the fetch_title.js script\nExiting program...");
        let body = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if config.debug {
            println!("{body}");
        }
        if body.contains(&config.to_check) {
            println!("{}: {}", config.social_name, "FAILED".red());
        } else {
            println!("{}: {}", config.social_name, request_url.green());
        }
    } else {
        if config.follow_redirs {
            let client = Client::builder()
                .redirect(reqwest::redirect::Policy::default())
                .build()?;
        }

        let response = reqwest::get(&request_url).await?;
        let body = response.text().await?;
        if config.debug {
            println!("{body}");
        }
        if body.contains(&config.to_check) {
            println!("{}: {}", config.social_name, "FAILED".red());
        } else {
            println!("{}: {}", config.social_name, request_url.green());
        }
    }

    Ok(())
}