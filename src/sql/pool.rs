use mysql::Pool;
use std::sync::OnceLock;

use super::{build_opts_from_env, EnvVars};

static POOL_INSTANCE: OnceLock<Pool> = OnceLock::new();

pub fn get_pool() -> &'static Pool {
    POOL_INSTANCE.get_or_init(|| {
        let env_vars = EnvVars::from_env();
        let opts = build_opts_from_env(env_vars);

        Pool::new(opts).expect("SqlConnectionError")
    })
}
