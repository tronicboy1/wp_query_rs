#[derive(Debug)]
pub enum CastType {
    Numeric,
    Binary,
    Char,
    Date,
    Datetime,
    Decimal,
    Signed,
    Time,
    Unsigned,
}

impl std::fmt::Display for CastType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Numeric => "NUMERIC",
                Self::Binary => "BINARY",
                Self::Char => "CHAR",
                Self::Date => "DATE",
                Self::Datetime => "DATETIME",
                Self::Decimal => "DECIMAL",
                Self::Signed => "SIGNED",
                Self::Time => "TIME",
                Self::Unsigned => "CHAR",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_convert_to_string() {
        assert_eq!(&CastType::Binary.to_string(), "BINARY")
    }
}
