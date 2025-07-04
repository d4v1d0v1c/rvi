use std::env;
use std::ffi::OsString;

pub fn get_args_from_env_vars() -> Vec<OsString> {
    [
        ("--ascii", "RMORE_ASCII"),
    ]
    .iter()
    .filter_map(|(flag, key)| {
        env::var(key)
            .ok()
            .map(|var| [flag.to_string(), var].join("="))
    })
    .map(|a| a.into())
    .collect()
}
