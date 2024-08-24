//
//  Struct definitions for target sites to check
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
        debug,
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
        debug,
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
        debug,

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
        debug,

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
        debug,

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
        debug,

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
        debug,

    )
}
