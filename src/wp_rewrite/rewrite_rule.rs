use std::collections::HashMap;

use crate::sql::find_col;

use super::WpRewrite;

#[derive(Debug)]
pub struct RewriteRule {
    /// Regular expression to match request against.
    pub regex: regex::Regex,
    /// The corresponding query vars for this rewrite rule.
    pub query: String,
    /// Number of parameters associated with a rewrite, used for finding most specific match
    param_count: usize,
    /// Priority of the new rule. Accepts 'top' or 'bottom'. Default 'bottom'.
    _after: Priority,
}

impl RewriteRule {
    /// Creates a query string from this rewrite rule from the original request. These are WP style paths created.
    ///
    /// `/author/admin` -> `author_name=admin`
    ///
    /// `/2023/11/my-post-name` -> `year=2023&monthnum=11&postname=my-post-name`
    pub fn replace(&self, path: &str) -> Option<String> {
        let keys = self.get_query_keys();

        self.regex.captures(path).map(|caps| {
            keys.into_iter()
                .enumerate()
                .fold(String::new(), |mut acc, (i, key)| {
                    if let Some(v) = caps.get(i + 1) {
                        if i > 0 {
                            acc += "&";
                        }

                        acc += &format!("{}={}", key, v.as_str());

                        acc
                    } else {
                        acc
                    }
                })
        })
    }

    /// Gets the query param keys for a given RewriteRule and returns them in order
    fn get_query_keys(&self) -> Vec<&'_ str> {
        let re = regex::Regex::new("[?&]([^=]+)=[^&#]*?").expect("CorrectRegex");

        re.captures_iter(&self.query)
            .filter_map(|c| c.get(1).map(|m| m.as_str()))
            .collect()
    }

    /// Checks if RewriteRule starts with a base, making it higher priority
    fn is_base(&self, wp_rewrite: &WpRewrite) -> bool {
        let regex_str = self.regex.as_str();
        regex_str.starts_with(&wp_rewrite.author_base)
            || regex_str.starts_with(&wp_rewrite.category_base)
            || regex_str.starts_with(&wp_rewrite.pagination_base)
            || regex_str.starts_with(&wp_rewrite.feed_base)
            || regex_str.starts_with(&wp_rewrite.search_base)
            || regex_str.starts_with(&wp_rewrite.comments_base)
    }
}

#[derive(Debug)]
enum Priority {
    _Top,
    Bottom,
}

#[derive(Debug)]
pub struct RewriteRules(Vec<RewriteRule>);

impl RewriteRules {
    /// Finds the most specific (having most parameters) RewriteRule that can be applied to a given URL path
    pub fn find_match(&self, path: &str, wp_rewrite: &WpRewrite) -> Option<&RewriteRule> {
        self.0
            .iter()
            .filter(|RewriteRule { regex, .. }| regex.is_match(path))
            .reduce(|acc, r| {
                // If regex starts with a WpRewrite base, prioritize it over
                let acc_is_base_match = acc.is_base(wp_rewrite);
                let next_is_base_match = r.is_base(wp_rewrite);
                // Find the most specific match, has the most param count
                let next_is_more_specific = r.param_count > acc.param_count;

                // use more specific base match if available
                if next_is_base_match && !acc_is_base_match
                    || next_is_base_match && next_is_more_specific
                {
                    return r;
                }

                if next_is_more_specific && !acc_is_base_match {
                    return r;
                }

                acc
            })
    }
}

impl mysql_common::prelude::FromRow for RewriteRules {
    fn from_row_opt(mut row: mysql_common::Row) -> Result<Self, mysql_common::FromRowError>
    where
        Self: Sized,
    {
        let data: String = find_col(&mut row, "option_value")
            .ok_or_else(|| mysql_common::FromRowError(row.clone()))?;

        data.try_into().map_err(|_| mysql_common::FromRowError(row))
    }
}

impl TryInto<RewriteRules> for String {
    type Error = serde_php::Error;

    fn try_into(self) -> Result<RewriteRules, Self::Error> {
        let data = serde_php::from_bytes::<HashMap<String, String>>(self.as_bytes())?;

        Ok(RewriteRules(
            data.into_iter()
                .filter_map(|(regex, query)| {
                    regex::Regex::new(&regex).ok().map(|regex| RewriteRule {
                        regex,
                        param_count: query.matches("$matches").count(),
                        query,
                        _after: Priority::Bottom,
                    })
                })
                .collect(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;

    use super::*;

    #[test]
    fn can_deserialize() {
        let db_res = std::fs::read_to_string("test_data/test_rewrite_rules.txt").unwrap();

        let rewrite_rules: RewriteRules = db_res.try_into().unwrap();

        assert_eq!(rewrite_rules.0.len(), 95);
    }

    #[test]
    fn can_get_query_key_value_pairs() {
        let rr = RewriteRule {
            regex: Regex::new("([0-9]{4})/([0-9]{1,2})/([^/]+)(?:/([0-9]+))?/?$").unwrap(),
            query: String::from(
                "index.php?year=$matches[1]&monthnum=$matches[2]&name=$matches[3]&page=$matches[4]",
            ),
            param_count: 4,
            _after: Priority::Bottom,
        };

        let key_value_pairs = rr.get_query_keys();

        assert_eq!(key_value_pairs.len(), 4);
        assert_eq!(key_value_pairs[0], "year");
        assert_eq!(key_value_pairs[1], "monthnum");
        assert_eq!(key_value_pairs[2], "name");
        assert_eq!(key_value_pairs[3], "page");
    }
}
