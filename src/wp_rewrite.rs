mod permalink_structure;
mod rewrite_code;
mod rewrite_filters;
mod rewrite_rule;

use std::{
    cell::{Ref, RefCell},
    ops::Deref,
};

use mysql::prelude::Queryable;
pub use rewrite_code::RewriteCode;
pub use rewrite_filters::RewriteFilters;

use crate::{sql::get_conn, ParamBuilder, Params, PostQueryable, PostType};

use self::{
    permalink_structure::PermalinkStructure, rewrite_filters::RewriteFilterCache,
    rewrite_rule::RewriteRules,
};

pub struct WpRewrite {
    /// The permalink structure as in the database. This is what you set on the Permalink Options page, and includes ‘tags’ like %year%, %month% and %post_id%.
    permalink_structure: PermalinkStructure,
    /// Anything to be inserted before category archive URLs. Defaults to ‘category/’.
    category_base: String,
    /// Structure for category archive URLs. This is just the $category_base plus ‘%category%’.
    category_structure: String,
    /// Anything to be inserted before author archive URLs. Defaults to ‘author/’.
    author_base: String,
    /// Structure for author archive URLs. This is just the $author_base plus ‘%author%’.
    author_structure: String,
    /// Anything to be inserted before pagination indices. Defaults to ‘page/’.
    pagination_base: String,
    /// Supported feeds names (rdf, rss, atom) Use add_feed to override or add.
    _feeds: String,
    /// Anything to be inserted before feed URLs. Defaults to ‘feed/’.
    _feed_base: String,
    /// Structure for feed URLs. This is just the $feed_base plus ‘%feed%’.
    _feed_structure: String,
    /// Anything to be inserted before searches. Defaults to ‘search/’.
    search_base: String,
    /// Structure for search URLs. This is just the $search_base plus ‘%search%’.
    search_structure: String,
    /// Anything to be inserted just before the $feed_structure to get the latest comments feed. Defaults to ‘comments’.
    comments_base: String,
    /// The structure for the latest comments feed. This is just $comments_base plus $feed_base plus ‘%feed%’.
    comments_feed_structure: String,
    /// Structure for dated archive URLs. Tries to be ‘%year%/%monthnum%/%day%’, ‘%day%/%monthnum%/%year%’ or ‘%monthnum%/%day%/%year%’,
    /// but if none of these are detected in your $permalink_structure, defaults to ‘%year%/%monthnum%/%day%’.
    /// Various functions use this structure to obtain less specific structures: for example, get_year_permastruct() simply
    /// removes the ‘%monthnum%’ and ‘%day%’ tags from $date_structure.
    date_structure: String,
    /// Structure for Pages. Just ‘%pagename%’.
    page_structure: String,
    /// Anything up to the start of the first tag in your $permalink_structure.
    front: String,
    /// The root of your WordPress install. Prepended to all structures.
    root: String,
    rules: RefCell<Option<RewriteRules>>,
    /// Know whether the rules were fetched from the DB or not, do not refetch if the results were bad/None
    rules_init: RefCell<bool>,

    /// Filters
    hooks: RewriteFilterCache,
}

impl WpRewrite {
    pub fn new() -> Self {
        Self {
            permalink_structure: PermalinkStructure::new(),
            category_base: String::new(),
            category_structure: String::new(),
            author_base: String::new(),
            author_structure: String::new(),
            pagination_base: String::new(),
            _feeds: String::new(),
            _feed_base: String::new(),
            _feed_structure: String::new(),
            search_base: String::new(),
            search_structure: String::new(),
            comments_base: String::new(),
            comments_feed_structure: String::new(),
            date_structure: String::new(),
            page_structure: String::new(),
            front: String::new(),
            root: String::new(),
            rules: RefCell::new(None),
            rules_init: RefCell::new(false),
            hooks: RewriteFilterCache::new(),
        }
    }

    /// Retrieves the rewrite rules from database.
    /// Results are cached if database result is valid
    pub fn wp_rewrite_rules(&self) -> Result<Ref<'_, Option<RewriteRules>>, mysql::Error> {
        let rules = self.rules.borrow();

        if rules.is_some() || *self.rules_init.borrow().deref() {
            return Ok(rules);
        }

        // Get rid of imutable borrow so we can mutate it
        drop(rules);

        let mut conn = get_conn()?;

        let res: Option<RewriteRules> = conn.exec_first(
            "SELECT option_value FROM wp_options WHERE option_name = 'rewrite_rules'",
            mysql::Params::Empty,
        )?;

        // SAFETY never borrows mut if it is already in the cache
        let mut rules_cache = self.rules.borrow_mut();
        *rules_cache = res;
        drop(rules_cache);

        Ok(self.rules.borrow())
    }
}

pub fn parse_request(
    wp_rewrite: &WpRewrite,
    url: url::Url,
) -> Result<url::Url, Box<dyn std::error::Error>> {
    let rules = wp_rewrite.wp_rewrite_rules()?;

    // If no rules, just return the query parameter p for post id
    if rules.is_none() && url.query().and_then(|q| q.find("p=")).is_some() {
        return Ok(url);
    }

    Ok(url)
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

#[derive(Debug)]
pub struct WpParseError {}

impl std::fmt::Display for WpParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not parse request")
    }
}

impl std::error::Error for WpParseError {}

trait ToRegex {
    fn to_regex(self) -> Result<regex::Regex, regex::Error>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add_filters() {
        let mut rewrite = WpRewrite::new();

        rewrite.add_filter().author_rewrite_rules(|mut rules| {
            rules.push(String::from("New Rule??"));
            rules
        });
    }

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
}