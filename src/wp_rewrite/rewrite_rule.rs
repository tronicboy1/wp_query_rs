use std::collections::HashMap;

use crate::sql::find_col;

pub struct RewriteRule {
    /// Regular expression to match request against.
    regex: regex::Regex,
    /// The corresponding query vars for this rewrite rule.
    query: String,
    /// Priority of the new rule. Accepts 'top' or 'bottom'. Default 'bottom'.
    _after: Priority,
}
enum Priority {
    Top,
    Bottom,
}

pub struct RewriteRules(Vec<RewriteRule>);

impl mysql_common::prelude::FromRow for RewriteRules {
    fn from_row_opt(mut row: mysql_common::Row) -> Result<Self, mysql_common::FromRowError>
    where
        Self: Sized,
    {
        let data: String = find_col(&mut row, "rewrite_rules")
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
    use super::*;

    #[test]
    fn can_deserialize() {
        let db_res = std::fs::read_to_string("test_data/test_rewrite_rules.txt").unwrap();

        let rewrite_rules: RewriteRules = db_res.try_into().unwrap();

        assert_eq!(rewrite_rules.0.len(), 99);
    }
}
