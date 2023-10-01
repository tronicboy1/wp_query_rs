use std::ops::Deref;

use crate::{ParamBuilder, Params, PostQueryable, PostType};

use super::WpRewrite;

pub fn parse_request(
    wp_rewrite: &WpRewrite,
    url: url::Url,
) -> Result<url::Url, Box<dyn std::error::Error>> {
    let rules = wp_rewrite.wp_rewrite_rules()?;

    let pathinfo = path_info(&url).to_string();
    // let mut req_uri = url.path().to_string();

    // if let Some(home_path_regex) = get_home_path_regex(home_path) {
    //     pathinfo = home_path_regex.replace(&pathinfo, "").to_string();
    //     req_uri = home_path_regex.replace(&req_uri, "").to_string();
    // }

    // let index_regex = regex::Regex::new("|^.*index.php$|").expect("ValidRegex");
    // let requested_path = if !pathinfo.is_empty() && !index_regex.is_match(&pathinfo) {
    //     pathinfo
    // } else {
    //     if req_uri.as_str() == "index.php" {
    //         String::new()
    //     } else {
    //         req_uri
    //     }
    // };

    // If no rules, just return the query parameter p for post id
    if let Some(rules) = rules.deref() {
        let matched_rule = rules.find_match(&pathinfo);
        dbg!(&matched_rule);
        if let Some(m) = matched_rule {
            let cap = m.regex.captures(&pathinfo);
            dbg!(&cap);
            if let Some(caps) = cap {
                let mut parsed = url.clone();
                parsed.set_path("index.php");


            }
        }
    } else if url.query().and_then(|q| q.find("p=")).is_some() {
        return Ok(url);
    }

    Err(Box::new(WpParseError {}))
}

#[derive(Debug)]
pub struct WpParseError {}

impl std::fmt::Display for WpParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not parse request")
    }
}

impl std::error::Error for WpParseError {}

/// Contains any client-provided pathname information trailing the actual script filename but preceding the query string, if available.
/// # Examples
/// ```rust,ignore
/// let url = url::Url::parse("http://www.example.com/php/path_info.php/some/stuff?foo=bar").unwrap();
///
/// let path_info = path_info(&url);
/// assert_eq!(path_info, "/some/stuff")
/// ```
fn path_info(url: &url::Url) -> &str {
    let path = url.path();

    let php_ext_i = path.find(".php").map(|i| {
        let start = ".php".chars().count() + i;
        &path[start..]
    });

    if let Some(path_after_php_ext) = php_ext_i {
        path_after_php_ext
    } else {
        path
    }
}

fn get_home_path_regex(home_path: Option<&str>) -> Option<regex::Regex> {
    home_path.and_then(|path| {
        let regex = String::from("|^%s|i");
        let regex = regex.replace("%s", path);

        regex::Regex::new(&regex).ok()
    })
}

impl<'a> TryFrom<url::Url> for Params<'a> {
    type Error = Box<dyn std::error::Error>;

    fn try_from(url_v: url::Url) -> Result<Self, Self::Error> {
        let mut params = ParamBuilder::new();

        for (key, value) in url_v.query_pairs() {
            match key.deref() {
                "p" => {
                    let p: u64 = value.parse()?;
                    params = params.p(p);
                }
                "post_type" => params = params.post_type(PostType::from(value.deref())),
                _ => {}
            }
        }

        Ok(params.into())
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use url::Url;

    use crate::wp_rewrite::rewrite_rule::RewriteRules;

    use super::*;

    #[test]
    fn can_rewrite_default() {
        let url = url::Url::parse("http://localhost:8080/?p=123").unwrap();

        let mut rewrite = WpRewrite::new();
        rewrite.rules_init = RefCell::new(true);

        let params: Params = parse_request(&rewrite, url)
            .and_then(|op| Params::try_from(op))
            .unwrap();

        assert_eq!(params.p, Some(123));
    }

    #[test]
    fn return_path_if_no_php() {
        let url = Url::parse("http://www.example.com/php/some/stuff?foo=bar").unwrap();
        let path_info = path_info(&url);

        assert_eq!(path_info, "/php/some/stuff")
    }

    #[test]
    fn return_path_after_php() {
        let url =
            Url::parse("http://www.example.com/php/path_info.php/some/stuff?foo=bar").unwrap();
        let path_info = path_info(&url);

        assert_eq!(path_info, "/some/stuff")
    }

    #[test]
    fn can_rewrite_blog_post() {
        let rewrite_rules = get_rewrite_dummy();
        let mut rewrite = WpRewrite::new();
        rewrite.rules = RefCell::new(Some(rewrite_rules));

        let url = Url::parse("http://localhost:8080/2023/09/my-test-meta-post-1695016100/").unwrap();

        let parsed = parse_request(&rewrite, url).unwrap();

        assert!(parsed
            .query_pairs()
            .find(|(key, v)| key == "p" && v == "164")
            .is_some());
    }

    fn get_rewrite_dummy() -> RewriteRules {
        let db_res = std::fs::read_to_string("test_data/test_rewrite_rules.txt").unwrap();
        db_res.try_into().unwrap()
    }
}
