// use crate::b64::b64u_decode;
use std::env;
use std::str::FromStr;

pub fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::MissingEnv(name))
}

pub fn get_env_parse<T: FromStr>(name: &'static str) -> Result<T> {
    let val = get_env(name)?;
    val.parse::<T>().map_err(|_| Error::WrongFormat(name))
}

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    MissingEnv(&'static str),
    WrongFormat(&'static str),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

//////////////////////////////////////////////////////

use std::sync::OnceLock;

pub fn load_env_file(test: bool) {
    if test {
        dotenv::from_filename(".env").unwrap();
    } else {
        dotenv::from_filename(".env.debug").unwrap();
    }
}

pub fn core_config() -> &'static CoreConfig {
    static INSTANCE: OnceLock<CoreConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        CoreConfig::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct CoreConfig {
    // -- Db
    pub DB_URL: String,
}

impl CoreConfig {
    fn load_from_env() -> Result<CoreConfig> {
        Ok(CoreConfig {
            // -- Db
            DB_URL: get_env("DATABASE_URL")?,
        })
    }
}
