use std::{borrow::Cow, ops::Deref};

use crate::{ParamBuilder, Params, PostQueryable, PostType};

use super::WpRewrite;

macro_rules! parse_request {
    ($url: ident, $rewrite: ident, $rules: expr) => {{
        let pathinfo = path_info(&$url).to_string();

        if let Some(rules) = $rules.deref() {
            let matched_rule = rules.find_match(&pathinfo, &$rewrite);
            if let Some(q_params) = matched_rule.and_then(|r| r.replace(&pathinfo)) {
                let mut parsed = $url.clone();
                parsed.set_path("index.php");
                parsed.set_query(Some(&q_params));

                return Ok(parsed);
            }
        } else if $url.query().and_then(|q| q.find("p=")).is_some() {
            // If is a default p=ID url, return it as is
            return Ok($url);
        }

        Err(Box::new(WpParseError {}))
    }};
}

#[cfg(feature = "query_sync")]
pub fn parse_request(
    wp_rewrite: &WpRewrite,
    url: url::Url,
) -> Result<url::Url, Box<dyn std::error::Error>> {
    parse_request!(url, wp_rewrite, { wp_rewrite.wp_rewrite_rules()? })
}
#[cfg(feature = "query_async")]
pub async fn parse_request(
    wp_rewrite: &WpRewrite,
    url: url::Url,
) -> Result<url::Url, Box<dyn std::error::Error>> {
    parse_request!(url, wp_rewrite, { wp_rewrite.wp_rewrite_rules().await? })
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

    let path = if let Some(path_after_php_ext) = php_ext_i {
        path_after_php_ext
    } else {
        path
    };

    // WP versions trims by '/' on front and end to get matches
    path.trim_matches('/')
}

fn _get_home_path_regex(home_path: Option<&str>) -> Option<regex::Regex> {
    home_path.and_then(|path| {
        let regex = String::from("|^%s|i");
        let regex = regex.replace("%s", path);

        regex::Regex::new(&regex).ok()
    })
}

impl<'a> TryFrom<&'a url::Url> for Params<'a> {
    type Error = Box<dyn std::error::Error>;

    fn try_from(url_v: &'a url::Url) -> Result<Self, Self::Error> {
        let mut params = ParamBuilder::new();

        for (key, value) in url_v.query_pairs() {
            let str = match value {
                Cow::Borrowed(v) => v,
                // SAFETY we are never modifying the Cow, so we know we can get the &str reference that lives 'a, i.e. the url that is being parsed
                _ => unreachable!("NeverModified"),
            };

            match key.deref() {
                "p" => {
                    let p: u64 = value.parse()?;
                    params = params.p(p);
                }
                "post_type" => params = params.post_type(PostType::from(str)),
                "year" => params = params.year(value.parse()?),
                "monthnum" => params = params.monthnum(value.parse()?),
                "name" | "pagename" => params = params.name(str),
                "category_name" => params = params.category_name(str),
                "author_name" => params = params.author_name(str),
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

    use super::*;

    #[test]
    #[cfg(feature = "query_sync")]
    fn can_rewrite_default() {
        let url = url::Url::parse("http://localhost:8080/?p=123").unwrap();

        let mut rewrite = WpRewrite::new();
        rewrite.rules_init = RefCell::new(true);

        let params = parse_request(&rewrite, url).unwrap();
        let params = Params::try_from(&params).unwrap();

        assert_eq!(params.p, Some(123));
    }

    #[test]
    fn return_path_if_no_php() {
        let url = Url::parse("http://www.example.com/php/some/stuff?foo=bar").unwrap();
        let path_info = path_info(&url);

        assert_eq!(path_info, "php/some/stuff")
    }

    #[test]
    fn return_path_after_php() {
        let url =
            Url::parse("http://www.example.com/php/path_info.php/some/stuff?foo=bar").unwrap();
        let path_info = path_info(&url);

        assert_eq!(path_info, "some/stuff")
    }

    #[test]
    #[cfg(feature = "query_sync")]
    fn can_rewrite_blog_post() {
        let rewrite = get_rewrite_dummy();

        let url =
            Url::parse("http://localhost:8080/2023/09/my-test-meta-post-1695016100/").unwrap();

        let parsed = parse_request(&rewrite, url).unwrap();

        assert!(parsed
            .query_pairs()
            .find(|(key, v)| key == "name" && v == "my-test-meta-post-1695016100")
            .is_some());
        assert!(parsed
            .query_pairs()
            .find(|(key, v)| key == "page")
            .is_none());
    }

    fn get_rewrite_dummy() -> WpRewrite {
        let db_res = std::fs::read_to_string("test_data/test_rewrite_rules.txt").unwrap();
        let rewrite_rules = db_res.try_into().unwrap();

        let mut rewrite = WpRewrite::new();
        rewrite.rules = RefCell::new(Some(rewrite_rules));

        rewrite
    }

    #[test]
    #[cfg(feature = "query_sync")]
    fn can_parse_request_into_params() {
        let rewrite = get_rewrite_dummy();

        let url =
            Url::parse("http://localhost:8080/2023/09/my-test-meta-post-1695016100/").unwrap();

        let parsed = parse_request(&rewrite, url).unwrap();

        let params = Params::try_from(&parsed).unwrap();

        assert_eq!(params.monthnum, Some(9));
        assert_eq!(params.year, Some(2023));
        assert_eq!(params.name, Some("my-test-meta-post-1695016100"));
    }

    #[test]
    #[cfg(feature = "query_sync")]
    fn can_parse_archive_page_into_params() {
        let rewrite = get_rewrite_dummy();

        let url = Url::parse("http://localhost:8080/category/derbies/").unwrap();

        let parsed = parse_request(&rewrite, url).unwrap();

        let params = Params::try_from(&parsed).unwrap();

        assert_eq!(params.term_slug_and, Some(vec!["derbies"]));
    }

    #[test]
    #[cfg(feature = "query_sync")]
    fn can_parse_author_page() {
        let rewrite = get_rewrite_dummy();

        let url = Url::parse("http://localhost:8080/author/admin/").unwrap();

        let parsed = parse_request(&rewrite, url).unwrap();

        let params = Params::try_from(&parsed).unwrap();

        assert_eq!(params.author_name, Some("admin"));
    }
}
