use std::{fmt::Display, str::FromStr};

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

#[cfg(test)]
mod tests {}
