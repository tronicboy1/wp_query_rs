use regex::Regex;

use super::ToRegex;

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
    Custom(Regex),
}

impl ToRegex for RewriteCode {
    fn to_regex(self) -> regex::Regex {
        match self {
            Self::Year => Regex::new("([0-9]{4})").unwrap(),
            Self::Monthnum => Regex::new("([0-9]{1,2})").unwrap(),
            Self::Day => Regex::new("([0-9]{1,2})").unwrap(),
            Self::Hour => Regex::new("([0-9]{1,2})").unwrap(),
            Self::Minute => Regex::new("([0-9]{1,2})").unwrap(),
            Self::Second => Regex::new("([0-9]{1,2})").unwrap(),
            Self::Postname => Regex::new("([^/]+)").unwrap(),
            Self::PostId => Regex::new("([0-9]+)").unwrap(),
            Self::Author => Regex::new("([^/]+)").unwrap(),
            Self::Pagename => Regex::new("([^/]+?)").unwrap(),
            Self::Search => Regex::new("(.+)").unwrap(),
            Self::Custom(r) => r,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_regex() {
        let code = RewriteCode::Monthnum;
        let regex = code.to_regex();

        let m = regex
            .find("12")
            .map(|m| -> u8 { m.as_str().parse().unwrap() });

        assert_eq!(m, Some(12));
    }
}
