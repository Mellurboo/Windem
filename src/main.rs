use std::env;
use colored::*;

mod r#impl {
    pub mod html_check;
    pub mod title_check;
    pub mod targets;
    pub mod write_result;
    pub mod websources;  // Import config module
}

use r#impl::write_result::write_result;
use r#impl::html_check::social_html_check;
use r#impl::title_check::social_title_check;
use r#impl::websources::target_site;
use r#impl::targets::{
    get_youtube_target,
    get_facebook_target,
    get_tiktok_target,
    get_twitch_target,
    get_flickr_target,
    get_github_target,
    get_bluesky_target,
    get_medaltv_target
};


fn print_banner() {
    println!("{}", "[]================================[]".green().bold());

    println!(
        "{}",
        "\
.____ ____ ____ ____ ____ ____ \n\
||F |||Y |||N |||D |||E |||M ||\n\
||__|||__|||__|||__|||__|||__||\n\
|/__\\|/__\\|/__\\|/__\\|/__\\|/__\\|\n\t@AvaLikesBread\n"
            .green()
            .bold()
    );
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!(
            "{}\nUsage: {} {}\n{}\n\t{}: debug",
            "ARGUMENT ERROR!".red().bold(),
            "fyndem".green().bold(),
            "<username>".yellow(),
            "Arguments".red().bold(),
            "-d: Debug".yellow()
        );
        std::process::exit(1);
    }

    let debug = args.contains(&"-d".to_string());

    print_banner();

    let user_name = &args[1];

    // list of the checks (add as you need it doenst matter)
    let youtube_check = get_youtube_target(user_name, debug);
    let facebook_check = get_facebook_target(user_name, debug);
    let tiktok_check = get_tiktok_target(user_name, debug);
    let twitch_check = get_twitch_target(user_name, debug);
    let flickr_check = get_flickr_target(user_name, debug);
    let github_check = get_github_target(user_name, debug);
    let bluesky_check = get_bluesky_target(user_name, debug);
    let medaltv_check = get_medaltv_target(user_name, debug);

    write_result(&format!("[-----{}-----]", user_name));

    // running checks, this behaves like a list, if its not here it doesnt get run
    social_title_check(&youtube_check).await;
    social_html_check(&facebook_check).await;
    social_title_check(&tiktok_check).await;
    social_html_check(&twitch_check).await;
    social_title_check(&flickr_check).await;
    social_title_check(&github_check).await;
    social_html_check(&bluesky_check).await;
    social_html_check(&medaltv_check).await;

    println!(
        "{}",
        "\nFyndem can't confirm some webpages due to technical limitations. Here are the links to check for yourself:"
            .yellow()
            .bold()
    );
    println!("X: https://x.com/{}/", user_name.yellow());
    println!("Reddit: https://www.reddit.com/user/{}/", user_name.yellow());
    println!("Tumblr: https://www.tumblr.com/{}", user_name.yellow());
}



// Mellurboo wasn't here <3
