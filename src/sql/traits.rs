#[cfg(feature = "query_sync")]
pub trait Insertable: Into<mysql::Params> {
    /// Consumes the object and inserts into the database.
    /// Returns the ID of the inserted object.
    fn insert(self) -> Result<u64, mysql::Error>;

    /// Consumes the iterable and inserts objects into database.
    /// This has better performance as it reuses the prepared statement.
    fn batch(values: impl IntoIterator<Item = Self>) -> Result<(), mysql::Error>;
}
