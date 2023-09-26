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

impl Into<WpOrderBy> for &str {
    fn into(self) -> WpOrderBy {
        match self {
            "ID" | "id" => WpOrderBy::ID,
            "post_author" | "author" => WpOrderBy::Author,
            "post_title" | "title" => WpOrderBy::Title,
            "post_name" | "name" | "slug" => WpOrderBy::Name,
            "post_type" | "type" => WpOrderBy::Type,
            "post_date" | "date" => WpOrderBy::Date,
            "post_modified" | "modified" => WpOrderBy::Modified,
            "post_parent" | "parent" => WpOrderBy::Parent,
            "comment_count" => WpOrderBy::CommentCount,
            "meta_value" => WpOrderBy::MetaValue,
            "meta_value_num" => WpOrderBy::MetaValueNum,
            _ => WpOrderBy::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_convert_from_str() {
        let str = "ID";
        let order: WpOrderBy = str.into();
        assert_eq!(order, WpOrderBy::ID);

        let str = "date";
        let order: WpOrderBy = str.into();
        assert_eq!(order, WpOrderBy::Date);

        let str = "slug";
        let order: WpOrderBy = str.into();
        assert_eq!(order, WpOrderBy::Name);

        let str = "modified";
        let order: WpOrderBy = str.into();
        assert_eq!(order, WpOrderBy::Modified);
    }
}
