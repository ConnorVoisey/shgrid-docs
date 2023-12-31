use dotenvy::dotenv;

#[derive(Debug)]
pub struct Config {
    pub port: u16,
}

impl Config {
    #[must_use]
    pub fn init() -> Config {
        // load env file as enviroment variables
        dotenv().expect("failed to read .env file");

        Config {
            port: get_env("PORT", None),
        }
    }
}

pub fn get_env<T: std::str::FromStr>(key: &str, default: Option<T>) -> T
where
    T::Err: std::fmt::Debug,
{
    match std::env::var(key) {
        Ok(val) => val
            .trim()
            .parse::<T>()
            .unwrap_or_else(|_| panic!("incorrect type for env '{key}'")),
        Err(err) => match default {
            Some(default_val) => default_val,
            None => panic!(
                "Missing required env variable {key}
Error: {err}"
            ),
        },
    }
}
