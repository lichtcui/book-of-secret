use crate::Secret;

use lazy_static::lazy_static;
use std::fs;

pub static FILE_PATH: &str = "assets/secret.yaml";

lazy_static! {
    pub static ref LOCAL_CACHE: Cache = Cache::init();
}

#[derive(Clone, Debug)]
pub struct Cache {
    pub configs: Vec<Secret>,
}

impl Cache {
    fn init() -> Self {
        let mut configs: Vec<Secret> = Vec::new();

        if let Some(result) = fs::read_to_string(FILE_PATH).ok() {
            result.split("---\n").into_iter().skip(1).for_each(|x| {
                configs.push(serde_yaml::from_str(&x).expect("failed to parse config"))
            });
        }

        Cache { configs }
    }
}
