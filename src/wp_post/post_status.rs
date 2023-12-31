use std::{fmt::Display, str::FromStr};

#[cfg(feature = "php")]
mod zval;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PostStatus {
    Publish,
    Pending,
    Draft,
    AutoDraft,
    Future,
    Private,
    Inherit,
    Trash,
    Any,
}

impl FromStr for PostStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = match s {
            "publish" => Self::Publish,
            "pending" => Self::Pending,
            "draft" => Self::Draft,
            "auto-draft" => Self::AutoDraft,
            "future" => Self::Future,
            "private" => Self::Private,
            "inherit" => Self::Inherit,
            "trash" => Self::Trash,
            _ => Self::Any,
        };

        Ok(v)
    }
}

impl Display for PostStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Publish => "publish",
                Self::Pending => "pending",
                Self::Draft => "draft",
                Self::AutoDraft => "auto-draft",
                Self::Future => "future",
                Self::Private => "private",
                Self::Inherit => "inherit",
                Self::Trash => "trash",
                Self::Any => "any",
            }
        )
    }
}

impl serde::Serialize for PostStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl Into<mysql_common::Value> for PostStatus {
    fn into(self) -> mysql_common::Value {
        mysql_common::Value::Bytes(self.to_string().into_bytes())
    }
}

impl Into<PostStatus> for String {
    fn into(self) -> PostStatus {
        PostStatus::from_str(&self).unwrap()
    }
}

impl mysql_common::prelude::FromValue for PostStatus {
    type Intermediate = String;
}

#[cfg(test)]
mod tests {
    use mysql_common::prelude::FromValue;

    use super::*;

    #[test]
    fn can_get_text_from_post_status() {
        assert_eq!(&PostStatus::Publish.to_string(), "publish");
        assert_eq!(&PostStatus::AutoDraft.to_string(), "auto-draft");
    }

    #[test]
    fn can_convert_from_str() {
        assert_eq!(
            PostStatus::from_str("publish").unwrap(),
            PostStatus::Publish
        );
        assert_eq!(PostStatus::from_str("future").unwrap(), PostStatus::Future);
    }

    #[test]
    fn can_format() {
        assert_eq!(&format!("{}", PostStatus::Pending), "pending");
        assert_eq!(&format!("{}", PostStatus::Private), "private");
    }

    #[test]
    fn can_convert_from_mysql_value() {
        let value = mysql_common::Value::Bytes(String::from("publish").into_bytes());

        let status = PostStatus::from_value(value);
        assert_eq!(PostStatus::Publish, status);
    }
}
