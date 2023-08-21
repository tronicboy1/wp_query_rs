#[derive(Debug)]
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

#[cfg(test)]
mod tests {}
