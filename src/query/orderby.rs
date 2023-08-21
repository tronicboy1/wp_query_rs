#[derive(Debug)]
pub enum WpOrderBy {
    None,
    ID,
    Author,
    Title,
    Name,
    Type,
    Date,
    Modified,
    Parent,
    CommentCount,
    MetaValue,
    MetaValueNum,
}

impl WpOrderBy {
    pub fn val(&self) -> &'static str {
        match self {
            Self::None => "",
            Self::ID => "ID",
            Self::Author => "post_author",
            Self::Title => "post_title",
            Self::Name => "post_name",
            Self::Type => "post_type",
            Self::Date => "post_date",
            Self::Modified => "post_modified",
            Self::Parent => "post_parent",
            Self::CommentCount => "comment_count",
            Self::MetaValue => "meta_value",
            Self::MetaValueNum => "meta_value",
        }
    }
}

#[cfg(test)]
mod tests {}
