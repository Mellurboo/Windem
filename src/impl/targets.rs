//
//  Struct definitions for target sites to check
//  this is the "List of websites" target being the target site for scraping
//
//  Arguments:
//      - Username to search for (needs formatting with link)
//      - Website URL to go before the username (they will concatenate later on)
//      - Name of the social platform
//      - What to search for to flag a 404 (profile not being found)
//      - Follow redirects (true/false)
//      - React to JavaScript (true/false) [true recommended]
//      - Reversed (true/false) [If it would normally return false, it returns positive]
//      - Debug
//      - How long to wait for website to load (Force a load. 0 = No forced load)
//

use crate::r#impl::websources::target_site;

pub fn get_youtube_target(user_name: &str, debug: bool) -> target_site {
    target_site::new(
        &format!("@{}", user_name),
        "https://www.youtube.com/",
        "YouTube",
        "404 Not Found",
        false,
        false,
        false,
        debug,
        0
    )
}

pub fn get_twitter_target(user_name: &str, debug: bool) -> target_site {
    target_site::new(
        user_name,
        "https://x.com/",
        "Twitter",
        user_name,
        true,
        true,
        true,
        debug,
        5000,
    )
}

pub fn get_facebook_target(user_name: &str, debug: bool) -> target_site {
    target_site::new(
        &format!("{}/", user_name),
        "https://www.facebook.com/",
        "Facebook",
        "This content isn't available at the moment",
        false,
        true,
        false,
        debug,
        0,
    )
}

pub fn get_tiktok_target(user_name: &str, debug: bool) -> target_site {
    target_site::new(
        &format!("@{}", user_name),
        "https://www.tiktok.com/",
        "TikTok",
        "Couldn’t find this account. Visit TikTok to discover more trending creators, hashtags, and sounds.",
        true,
        true,
        false,
        debug,
        0,
    )
}

pub fn get_twitch_target(user_name: &str, debug: bool) -> target_site {
    target_site::new(
        user_name,
        "https://www.twitch.tv/",
        "Twitch",
        "Sorry. Unless you've got a time machine, that content is unavailable.",
        false,
        true,
        false,
        debug,
        0,
    )
}

pub fn get_flickr_target(user_name: &str, debug: bool) -> target_site {
    target_site::new(
        user_name,
        "https://www.flickr.com/photos/",
        "Flickr",
        "Oops! | Flickr",
        false,
        true,
        false,
        debug,
        0,
    )
}

pub fn get_github_target(user_name: &str, debug: bool) -> target_site {
    target_site::new(
        user_name,
        "https://github.com/",
        "Github",
        "Page not found",
        false,
        true,
        false,
        debug,
        0,
    )
}

pub fn get_bluesky_target(user_name: &str, debug: bool) -> target_site {
    target_site::new(
        &format!("{}.bsky.social", user_name),
        "https://bsky.app/profile/",
        "Bluesky",
        "Unable to resolve handle",
        false,
        true,
        false,
        debug,
        0,
    )
}

pub fn get_medaltv_target(user_name: &str, debug: bool) -> target_site {
    target_site::new(
        user_name,
        "https://medal.tv/u/",
        "MedalTV",
        "That user does not exist.",
        false,
        true,
        false,
        debug,
        0,
    )
}
