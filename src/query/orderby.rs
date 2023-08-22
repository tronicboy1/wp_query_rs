#[derive(Debug, PartialEq, Eq)]
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

impl ToString for WpOrderBy {
    fn to_string(&self) -> String {
        let str = match self {
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
        };

        str.to_string()
    }
}

#[cfg(test)]
mod tests {}
