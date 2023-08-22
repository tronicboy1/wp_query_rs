use std::env;

#[derive(Debug)]
pub struct EnvVars {
    pub host: Option<String>,
    pub user: Option<String>,
    pub password: Option<String>,
    pub db_name: Option<String>,
    pub port: Option<u16>,
}

enum EnvVarKeys {
    Host,
    User,
    Password,
    DbName,
    Port,
}

impl EnvVarKeys {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Host => "WORDPRESS_DB_HOST",
            Self::User => "WORDPRESS_DB_USER",
            Self::Password => "WORDPRESS_DB_PASSWORD",
            Self::DbName => "WORDPRESS_DB_NAME",
            Self::Port => "WORDPRESS_DB_PORT",
        }
    }

    fn get_env(&self) -> Option<String> {
        env::var(self.as_str()).ok()
    }
}

impl EnvVars {
    pub fn new(
        host: String,
        user: String,
        password: String,
        db_name: String,
        port: Option<u16>,
    ) -> EnvVars {
        EnvVars {
            host: Some(host),
            user: Some(user),
            password: Some(password),
            db_name: Some(db_name),
            port,
        }
    }

    pub fn from_env<'a>() -> EnvVars {
        let host = EnvVarKeys::Host.get_env();
        let user = EnvVarKeys::User.get_env();
        let password = EnvVarKeys::Password.get_env();
        let db_name = EnvVarKeys::DbName.get_env();
        let port = EnvVarKeys::Port
            .get_env()
            .map(|str| str.trim().parse().ok())
            .flatten();

        EnvVars {
            host,
            user,
            password,
            db_name,
            port,
        }
    }
}
