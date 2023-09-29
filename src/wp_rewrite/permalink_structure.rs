use crate::sql::find_col;

use super::RewriteCode;

#[derive(Debug, PartialEq, Eq)]
pub struct PermalinkStructure {
    /// When none, defaults to /?p=123
    rewrite_codes: Option<Vec<RewriteCode>>,
}

impl PermalinkStructure {
    pub fn new() -> Self {
        Self {
            rewrite_codes: None,
        }
    }
}

impl TryInto<PermalinkStructure> for &str {
    type Error = PermalinkStructureParseError;
    fn try_into(self) -> Result<PermalinkStructure, Self::Error> {
        let parts = self.split('/').filter(|p| p.len() > 0);

        let rewrite_codes: Result<Vec<RewriteCode>, super::rewrite_code::RewriteCodeError> =
            parts.map(|p| p.try_into()).collect();

        rewrite_codes
            .map(|codes| PermalinkStructure {
                rewrite_codes: Some(codes),
            })
            .map_err(|e| PermalinkStructureParseError(e.to_string()))
    }
}

impl mysql_common::prelude::FromRow for PermalinkStructure {
    fn from_row_opt(mut row: mysql_common::Row) -> Result<Self, mysql_common::FromRowError>
    where
        Self: Sized,
    {
        let s: String = find_col(&mut row, "option_value")
            .ok_or_else(|| mysql_common::FromRowError(row.clone()))?;

        s.as_str()
            .try_into()
            .map_err(|_| mysql_common::FromRowError(row))
    }
}

#[derive(Debug)]
pub struct PermalinkStructureParseError(String);

impl std::fmt::Display for PermalinkStructureParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not parse structure: {}", self.0)
    }
}

impl std::error::Error for PermalinkStructureParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_convert_from_mysql_value() {
        let s = String::from("/%year%/%monthnum%/%postname%/");

        let sructure = <&str as TryInto<PermalinkStructure>>::try_into(s.as_str());

        assert_eq!(
            sructure.unwrap(),
            PermalinkStructure {
                rewrite_codes: Some(vec![
                    RewriteCode::Year,
                    RewriteCode::Monthnum,
                    RewriteCode::Postname
                ])
            }
        )
    }
}
