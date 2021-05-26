use std::{env, str::FromStr};

pub struct Config {
    pub l2_size: usize,
    pub slab_to_cache_ration: f32,
    pub cache_hits_per_s: u64,
    pub debug: bool,
}

fn parse_env_var<T>(name: &str) -> Result<T, String>
where
    T: FromStr + ToString,
{
    std::env::var(name)
        .map_err(|_| format!("Env variable {} is missing", name))?
        .parse::<T>()
        .map_err(|_| {
            format!(
                "Env variable {} should be an {}",
                name,
                std::any::type_name::<T>()
            )
        })
}

fn parse_env_var_or<T>(name: &str, default: T) -> Result<T, String>
where
    T: FromStr + ToString,
{
    Ok(match env::var(&name) {
        Err(env::VarError::NotPresent) => {
            println!(
                "Env variable {} is missing, defaults to {}",
                name,
                default.to_string()
            );
            default
        }
        Err(env::VarError::NotUnicode(_)) => {
            return Err(format!(
                "Env variable {} should be an {}",
                name,
                std::any::type_name::<T>()
            ));
        }
        Ok(v) => v.parse().map_err(|_| {
            format!(
                "Env variable {} should be an {}",
                name,
                std::any::type_name::<T>()
            )
        })?,
    })
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let l2_size: usize = parse_env_var("L2_SIZE")?;
        let cache_hits_per_s: u64 = parse_env_var_or("L3_HITS", 100_000_000)?;
        let debug = env::var("DEBUG").map(|v| v == "true").unwrap_or(false);
        let slab_to_cache_ration: f32 = parse_env_var_or("SLAB_CACHE_RATIO", 2.0)?;

        Ok(Self {
            l2_size,
            cache_hits_per_s,
            slab_to_cache_ration,
            debug,
        })
    }
}
