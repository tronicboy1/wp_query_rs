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

impl<'a> Into<Vec<u8>> for PostType<'a> {
    fn into(self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}

impl<'a> From<&'a str> for PostType<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            "post" => Self::Post,
            "page" => Self::Page,
            "revision" => Self::Revision,
            "attachment" => Self::Attachment,
            "nav_menu_item" => Self::NavMenuItem,
            _ => Self::CustomPostType(value),
        }
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
