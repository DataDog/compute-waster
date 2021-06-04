use crate::dstatsd;
use std::cell::RefCell;
use std::{env, str::FromStr};

pub struct Config {
    pub l2_size: usize,
    pub slab_to_cache_ration: f32,
    pub cache_hits_per_s: u64,
    pub debug: bool,
    pub sleep_duration: u64, // In micro seconds
    pub dd_client: Option<RefCell<dstatsd::Client>>,
    pub dd_tags: String,
    pub dd_metric_name: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let l2_size: usize = parse_env_var("L2_SIZE")?;
        let cache_hits_per_s: u64 = parse_env_var_or("L3_HITS", 100_000_000)?;
        let debug = env::var("DEBUG").map(|v| v == "true").unwrap_or(false);
        let slab_to_cache_ration: f32 = parse_env_var_or("SLAB_CACHE_RATIO", 2.0)?;
        let sleep_duration: u64 = parse_env_var_or("SLEEP_DURATION", 100)?;

        let client_url = env::var("STATSD_URL");
        let statsd_enable = env::var("STATSD_ENABLE")
            .map(|v| v == "true")
            .unwrap_or(false);
        let dd_client = if statsd_enable {
            match client_url {
                Ok(addr) => {
                    let client = RefCell::new(
                        dstatsd::Client::new(addr.clone()).map_err(|e| e.to_string())?,
                    );
                    println!(
                        "Instantiated dogstatsd client connected to STATSD_URL={}",
                        addr
                    );
                    Some(client)
                }
                Err(_e) => {
                    return Err("Env variable STATSD_ENABLE but STATSD_URL is unreadable".into());
                }
            }
        } else {
            None
        };

        let dd_tags = env::var("DD_TAGS").unwrap_or("".to_string());
        let dd_metric_name =
            env::var("DD_METRIC_NAME").unwrap_or("apm_reliability.l3_daemonset".to_string());

        Ok(Self {
            l2_size,
            cache_hits_per_s,
            slab_to_cache_ration,
            debug,
            sleep_duration,
            dd_client,
            dd_tags,
            dd_metric_name,
        })
    }
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
