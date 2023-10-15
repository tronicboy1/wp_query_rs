#[cfg(feature = "query_async")]
use std::{future::Future, pin::Pin};

#[cfg(feature = "query_sync")]
pub trait Insertable: Into<mysql::Params> {
    /// Consumes the object and inserts into the database.
    /// Returns the ID of the inserted object.
    fn insert(self) -> Result<u64, mysql::Error>;

    /// Consumes the iterable and inserts objects into database.
    /// This has better performance as it reuses the prepared statement.
    fn batch(values: impl IntoIterator<Item = Self>) -> Result<(), mysql::Error>
    where
        Self: Sized;
}

#[cfg(feature = "query_async")]
pub trait Insertable: Into<mysql_async::Params> {
    /// Consumes the object and inserts into the database.
    /// Returns the ID of the inserted object.
    fn insert(self) -> Pin<Box<dyn Future<Output = Result<u64, mysql_async::Error>>>>;

    /// Consumes the iterable and inserts objects into database.
    /// This has better performance as it reuses the prepared statement.
    fn batch<T>(values: T) -> Pin<Box<dyn Future<Output = Result<(), mysql_async::Error>>>>
    where
        T: IntoIterator<Item = Self> + Send + 'static,
        T::IntoIter: Send,
        Self: Sized;
}
