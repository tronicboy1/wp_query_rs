#[derive(Debug)]
pub enum Orderby {
    Agent,
    Approved,
    Author,
    AuthorEmail,
    AuthorIp,
    AuthorUrl,
    Content,
    Date,
    DateGmt,
    Id,
    Karma,
    Parent,
    PostId,
    Type,
    UserId,
    MetaValue(String),
    MetaKey(String),
    None,
}

impl std::fmt::Display for Orderby {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Agent => "comment_agent",
                Self::Approved => "comment_approved",
                Self::Author => "comment_author",
                Self::AuthorEmail => "comment_author_email",
                Self::AuthorIp => "comment_author_IP",
                Self::AuthorUrl => "comment_author_url",
                Self::Content => "comment_content",
                Self::Date => "comment_date",
                Self::DateGmt => "comment_date_gmt",
                Self::Id => "comment_ID",
                Self::Karma => "comment_karma",
                Self::MetaKey(s) => s.as_str(),
                Self::MetaValue(s) => s.as_str(),
                Self::None => "",
                Self::Parent => "comment_parent",
                Self::PostId => "comment_post_ID",
                Self::Type => "comment_type",
                Self::UserId => "user_id",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_to_string() {
        assert_eq!(
            Orderby::DateGmt.to_string(),
            String::from("comment_date_gmt")
        )
    }
}
