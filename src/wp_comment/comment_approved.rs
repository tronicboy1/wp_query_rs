#[derive(Debug, PartialEq, Eq)]
pub enum CommentApproved {
    Approved,
    Hold,
    All,
    Custom(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create() {
        let c_type = CommentApproved::Custom(String::from("Hello World"));

        match c_type {
            CommentApproved::Custom(s) => assert_eq!(&s, "Hello World"),
            _ => panic!("Did not match!"),
        }
    }
}
