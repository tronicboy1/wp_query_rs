#[cfg(feature = "query_sync")]
use mysql::Pool;
#[cfg(feature = "query_async")]
use mysql_async::Pool;
use std::sync::OnceLock;

use super::{build_opts_from_env, EnvVars};

// SAFETY okay to use OnceLock in async as there is no internal thread blocking
// and OnceLock implements Sync
static POOL_INSTANCE: OnceLock<Pool> = OnceLock::new();

pub fn get_pool() -> &'static Pool {
    POOL_INSTANCE.get_or_init(|| {
        let env_vars = EnvVars::from_env();
        let opts = build_opts_from_env(env_vars);

        Pool::new(opts)
    })
}
