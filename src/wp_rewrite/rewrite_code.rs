use std::error::Error;

use regex::Regex;

use super::ToRegex;

#[derive(Debug, PartialEq, Eq)]
pub enum RewriteCode {
    Year,
    Monthnum,
    Day,
    Hour,
    Minute,
    Second,
    Postname,
    PostId,
    Author,
    Pagename,
    Search,
    Category,
    Tag,
    Custom(String),
}

impl ToRegex for RewriteCode {
    /// # Err
    /// Will err if Custom rewrite code has invalid Regex registered
    fn to_regex(self) -> Result<regex::Regex, regex::Error> {
        match self {
            Self::Year => Regex::new("([0-9]{4})"),
            Self::Monthnum => Regex::new("([0-9]{1,2})"),
            Self::Day => Regex::new("([0-9]{1,2})"),
            Self::Hour => Regex::new("([0-9]{1,2})"),
            Self::Minute => Regex::new("([0-9]{1,2})"),
            Self::Second => Regex::new("([0-9]{1,2})"),
            Self::Postname => Regex::new("([^/]+)"),
            Self::PostId => Regex::new("([0-9]+)"),
            Self::Category => Regex::new("([^/]+)"),
            Self::Author => Regex::new("([^/]+)"),
            Self::Pagename => Regex::new("([^/]+?)"),
            Self::Search => Regex::new("(.+)"),
            Self::Tag => Regex::new("([^/]+)"),
            Self::Custom(r) => Regex::new(&r),
        }
    }
}

impl TryInto<RewriteCode> for &str {
    type Error = RewriteCodeError;
    fn try_into(self) -> Result<RewriteCode, Self::Error> {
        match self {
            "%year%" => Ok(RewriteCode::Year),
            "%monthnum%" => Ok(RewriteCode::Monthnum),
            "%day%" => Ok(RewriteCode::Day),
            "%hour%" => Ok(RewriteCode::Hour),
            "%minute%" => Ok(RewriteCode::Minute),
            "%second%" => Ok(RewriteCode::Second),
            "%post_id%" => Ok(RewriteCode::PostId),
            "%postname%" => Ok(RewriteCode::Postname),
            "%category%" => Ok(RewriteCode::Category),
            "%author%" => Ok(RewriteCode::Author),
            "%tag%" => Ok(RewriteCode::Tag),
            _ => Err(RewriteCodeError(self.to_string())),
        }
    }
}

#[derive(Debug)]
pub struct RewriteCodeError(String);

impl std::fmt::Display for RewriteCodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not convert rewrite code: {}", self.0)
    }
}

impl Error for RewriteCodeError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_regex() {
        let code = RewriteCode::Monthnum;
        let regex = code.to_regex();

        let m = regex
            .unwrap()
            .find("12")
            .map(|m| -> u8 { m.as_str().parse().unwrap() });

        assert_eq!(m, Some(12));
    }

    #[test]
    fn can_convert_from_string() {
        assert_eq!(
            <&str as TryInto<RewriteCode>>::try_into("%year%").unwrap(),
            RewriteCode::Year
        );
        assert_eq!(
            <&str as TryInto<RewriteCode>>::try_into("%monthnum%").unwrap(),
            RewriteCode::Monthnum
        );
        assert_eq!(
            <&str as TryInto<RewriteCode>>::try_into("%day%").unwrap(),
            RewriteCode::Day
        );
        assert_eq!(
            <&str as TryInto<RewriteCode>>::try_into("%hour%").unwrap(),
            RewriteCode::Hour
        );
        assert_eq!(
            <&str as TryInto<RewriteCode>>::try_into("%minute%").unwrap(),
            RewriteCode::Minute
        );
        assert_eq!(
            <&str as TryInto<RewriteCode>>::try_into("%second%").unwrap(),
            RewriteCode::Second
        );
        assert_eq!(
            <&str as TryInto<RewriteCode>>::try_into("%post_id%").unwrap(),
            RewriteCode::PostId
        );
        assert_eq!(
            <&str as TryInto<RewriteCode>>::try_into("%postname%").unwrap(),
            RewriteCode::Postname
        );
        assert_eq!(
            <&str as TryInto<RewriteCode>>::try_into("%category%").unwrap(),
            RewriteCode::Category
        );
        assert_eq!(
            <&str as TryInto<RewriteCode>>::try_into("%author%").unwrap(),
            RewriteCode::Author
        );
        assert_eq!(
            <&str as TryInto<RewriteCode>>::try_into("%tag%").unwrap(),
            RewriteCode::Tag
        );
    }
}
