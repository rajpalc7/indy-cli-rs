extern crate serde_json;

pub mod common;
pub mod did;
pub mod pool;
pub mod wallet;
pub mod ledger;

use std::collections::HashMap;
use IndyContext;

pub fn get_str_param<'a>(name: &'a str, params: &'a HashMap<&'static str, &str>) -> Result<&'a str, ()> {
    match params.get(name) {
        Some(v) => Ok(*v),
        None => {
            println_err!("No required \"{}\" parameter present", name);
            Err(())
        }
    }
}

pub fn get_opt_str_param<'a>(key: &'a str, params: &'a HashMap<&'static str, &str>) -> Result<Option<&'a str>, ()> {
    Ok(params.get(key).map(|v| *v))
}

#[allow(dead_code)] // FIXME
pub fn get_i64_param(name: &str, params: &HashMap<&'static str, &str>) -> Result<i64, ()> {
    match params.get(name) {
        Some(v) => {
            Ok(v.parse::<i64>().map_err(|err|
                println_err!("Can't parse integer parameter \"{}\": err {}", name, err))?)
        }
        None => {
            println_err!("No required \"{}\" parameter present", name);
            Err(())
        }
    }
}

pub fn get_opt_i64_param(key: &str, params: &HashMap<&'static str, &str>) -> Result<Option<i64>, ()> {
    let res = match params.get(key) {
        Some(value) => Some(value.parse::<i64>().map_err(|err|
            println_err!("Can't parse integer parameter \"{}\": err {}", key, err))?),
        None => None
    };
    Ok(res)
}

pub fn get_i32_param(name: &str, params: &HashMap<&'static str, &str>) -> Result<i32, ()> {
    match params.get(name) {
        Some(v) => {
            Ok(v.parse::<i32>().map_err(|err|
                println_err!("Can't parse integer parameter \"{}\": err {}", name, err))?)
        }
        None => {
            println_err!("No required \"{}\" parameter present", name);
            Err(())
        }
    }
}

pub fn get_opt_i32_param(name: &str, params: &HashMap<&'static str, &str>) -> Result<Option<i32>, ()> {
    match params.get(name) {
        Some(v) => {
            Ok(Some(v.parse::<i32>().map_err(|err|
                println_err!("Can't parse integer parameter \"{}\": err {}", name, err))?))
        }
        None => Ok(None)
    }
}

pub fn get_opt_bool_param(key: &str, params: &HashMap<&'static str, &str>) -> Result<Option<bool>, ()> {
    let res = match params.get(key) {
        Some(value) => Some(value.parse::<bool>().map_err(|err|
            println_err!("Can't parse bool parameter \"{}\": err {}", key, err))?),
        None => None
    };
    Ok(res)
}

pub fn get_str_array_param<'a>(name: &'a str, params: &'a HashMap<&'static str, &str>) -> Result<Vec<&'a str>, ()> {
    match params.get(name) {
        Some(v) => Ok(v.split(",").collect::<Vec<&'a str>>()),
        None => Err(println_err!("No required \"{}\" parameter present", name))
    }
}

pub fn get_opt_str_array_param<'a>(name: &'a str, params: &'a HashMap<&'static str, &str>) -> Result<Option<Vec<&'a str>>, ()> {
    match params.get(name) {
        Some(v) => Ok(Some(v.split(",").collect::<Vec<&'a str>>())),
        None => Ok(None)
    }
}

pub fn get_object_param<'a>(name: &'a str, params: &'a HashMap<&'static str, &str>) -> Result<serde_json::Value, ()> {
    match params.get(name) {
        Some(v) => Ok(serde_json::from_str(*v).map_err(|err|
            println_err!("Can't parse object parameter \"{}\": err {}", name, err))?),
        None => {
            println_err!("No required \"{}\" parameter present", name);
            Err(())
        }
    }
}

pub fn get_active_did(ctx: &IndyContext) -> Result<String, ()> {
    match ctx.get_active_did() {
        Some(did) => Ok(did),
        None => Err(println_err!("There is no active did"))
    }
}

pub fn get_opened_wallet_handle(ctx: &IndyContext) -> Result<i32, ()> {
    match ctx.get_opened_wallet_handle() {
        Some(wallet_handle) => Ok(wallet_handle),
        None => Err(println_err!("There is no opened wallet now"))
    }
}

pub fn get_connected_pool_handle(ctx: &IndyContext) -> Result<i32, ()> {
    match ctx.get_connected_pool_handle() {
        Some(pool_handle) => Ok(pool_handle),
        None => Err(println_err!("There is no opened pool now"))
    }
}