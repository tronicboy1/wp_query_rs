#[derive(Debug, PartialEq, Eq)]
pub enum CommentType {
    Comment,
    Pingback,
    Trackback,
    Custom(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create() {
        let c_type = CommentType::Custom(String::from("Hello World"));

        match c_type {
            CommentType::Custom(s) => assert_eq!(&s, "Hello World"),
            _ => panic!("Did not match!"),
        }
    }
}
