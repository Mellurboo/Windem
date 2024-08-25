//
//  Structs for building a site target makes it really easy to just add one,
//  aslong as you know either to make it check HTML or TITLE
//

pub struct target_site {
    pub user_name: String,
    pub link: String,
    pub social_name: String,
    pub to_check: String,
    pub follow_redirs: bool,
    pub react_to_js: bool,
    pub reversed: bool,
    pub debug: bool,
    pub delay: i32,
}

impl target_site {
    pub fn new(
        user_name: &str,
        link: &str,
        social_name: &str,
        to_check: &str,
        follow_redirs: bool,
        react_to_js: bool,
        reversed: bool,
        debug: bool,
        delay: i32,
    ) -> Self {
        target_site {
            user_name: user_name.to_string(),
            link: link.to_string(),
            social_name: social_name.to_string(),
            to_check: to_check.to_string(),
            follow_redirs,
            react_to_js,
            reversed,
            debug,
            delay,
        }
    }
}
