#[derive(Debug, PartialEq, Eq)]
pub enum PostType<'a> {
    Post,
    Page,
    Revision,
    Attachment,
    NavMenuItem,
    CustomPostType(&'a str),
}

impl<'a> std::fmt::Display for PostType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Post => "post",
                Self::Page => "page",
                Self::Revision => "revision",
                Self::Attachment => "attachment",
                Self::NavMenuItem => "nav_menu_item",
                Self::CustomPostType(s) => s,
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create() {
        let pt = PostType::Page;
        assert_eq!(pt, PostType::Page);
    }

    #[test]
    fn can_create_cpt() {
        let cpt = PostType::CustomPostType("cpt");

        match cpt {
            PostType::CustomPostType(s) => assert_eq!(s, "cpt"),
            _ => panic!("Did not match"),
        }
    }
}
