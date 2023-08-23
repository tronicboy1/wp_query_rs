use std::{fmt::Display, str::FromStr};

use ext_php_rs::convert::IntoZval;

#[derive(Debug, PartialEq, Eq)]
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

impl PostStatus {
    pub fn val(&self) -> &'static str {
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
    }
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
            other => Self::Any,
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

impl IntoZval for PostStatus {
    const TYPE: ext_php_rs::flags::DataType = ext_php_rs::flags::DataType::String;

    fn into_zval(self, persistent: bool) -> ext_php_rs::error::Result<ext_php_rs::types::Zval> {
        self.to_string().into_zval(persistent)
    }

    fn set_zval(
        self,
        zv: &mut ext_php_rs::types::Zval,
        persistent: bool,
    ) -> ext_php_rs::error::Result<()> {
        zv.set_string(&self.to_string(), persistent)
    }
}

#[cfg(test)]
mod tests {}
