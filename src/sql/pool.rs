#[cfg(feature = "query_sync")]
use mysql::Pool;
#[cfg(feature = "query_async")]
use mysql_async::Pool;
use std::sync::OnceLock;

use super::{build_opts_from_env, EnvVars};

// SAFETY okay to use OnceLock in async as there is no internal thread blocking
// and OnceLock implements Sync
static POOL_INSTANCE: OnceLock<Pool> = OnceLock::new();

#[cfg(feature = "query_sync")]
pub fn get_pool() -> &'static Pool {
    POOL_INSTANCE.get_or_init(|| {
        let env_vars = EnvVars::from_env();
        let opts = build_opts_from_env(env_vars);

        Pool::new(opts).expect("DatabaseConnectionFailed")
    })
}

#[cfg(feature = "query_async")]
pub fn get_pool() -> &'static Pool {
    POOL_INSTANCE.get_or_init(|| {
        let env_vars = EnvVars::from_env();
        let opts = build_opts_from_env(env_vars);

        Pool::new(opts)
    })
}

pub struct PoolInit;

impl PoolInit {
    /// Initializes the global mysql connection pool with options prepared by the package user.
    ///
    /// # Example
    /// ```rust
    /// use wp_query_rs::PoolInit;
    /// use mysql_async::OptsBuilder;
    ///
    /// let opts = OptsBuilder::default()
    /// .user(Some("root"))
    /// .ip_or_hostname("localhost")
    /// .pass(Some("password"))
    /// .db_name(Some("wordpress"));
    ///
    /// PoolInit::with_opts(opts.into());
    /// ```
    ///
    /// # Errors
    /// Will error if you call a WpQuery database function before initializing the database as the default functionality is to
    /// use environment variables to initialize a pool.
    #[cfg(feature = "query_async")]
    pub fn with_opts(opts: mysql_async::Opts) -> Result<(), Pool> {
        POOL_INSTANCE.set(Pool::new(opts))
    }

    /// Initializes the global mysql connection pool with options prepared by the package user.
    ///
    /// # Example
    /// ```rust,ignore
    /// use wp_query_rs::PoolInit;
    /// use mysql::OptsBuilder;
    ///
    /// let opts = OptsBuilder::default()
    /// .user(Some("root"))
    /// .ip_or_hostname("localhost")
    /// .pass(Some("password"))
    /// .db_name(Some("wordpress"));
    ///
    /// PoolInit::with_opts(opts.into());
    /// ```
    ///
    /// # Errors
    /// Will error if you call a WpQuery database function before initializing the database as the default functionality is to
    /// use environment variables to initialize a pool.
    #[cfg(feature = "query_sync")]
    pub fn with_opts(opts: mysql::Opts) -> Result<(), Pool> {
        POOL_INSTANCE.set(Pool::new(opts).expect("DatabaseConnectionFailed"))
    }

    /// Initializes the global pool with a reference to an already configured pool.
    #[cfg(feature = "query_async")]
    pub fn with_pool(pool: &mysql_async::Pool) -> Result<(), Pool> {
        // Cloning just creates a new Arc::clone to the actual pool provided, so there is no major performance penalty here.
        // The reference provided in the params does not need to be 'static because Arc will hold on to the pool as long as necessary.
        POOL_INSTANCE.set(pool.clone())
    }

    /// Initializes the global pool with a reference to an already configured pool.
    #[cfg(feature = "query_sync")]
    pub fn with_pool(pool: &mysql::Pool) -> Result<(), Pool> {
        // Cloning just creates a new Arc::clone to the actual pool provided, so there is no major performance penalty here.
        // The reference provided in the params does not need to be 'static because Arc will hold on to the pool as long as necessary.
        POOL_INSTANCE.set(pool.clone())
    }
}
