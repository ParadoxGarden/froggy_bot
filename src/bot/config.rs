use core::panic;
use serde::Deserialize;
use std::env;
use std::fmt;
use std::fs;

pub fn get_config() -> Config {
    let default_config_file: &str = "./config.json";
    // settings precedence
    // 1. first try to read env vars
    // 2. then try the config file var
    // 3. then default config file location

    // 1 comes last when I get all the values figured out
    let env_config: &str = "FROGGY_CONFIG";

    let user_file_path: &str = ""; // user defined

    // 2
    let user_file_path = match env::var(env_config) {
        Ok(success) => {
            println!("{env_config}: {user_file_path:?}");
            success
        }
        Err(_e) => {
            println!(
                "couldn't read '${env_config}'! trying default config at {default_config_file}"
            );
            default_config_file.to_string()
        }
    };
    //precedence?? maybe set boolean
    let contents: String = match fs::read_to_string(user_file_path) {
        Ok(success) => {
            println!("we read the config file correctyl");
            success
        }
        Err(e) => {
            println!("no config file found");
            panic!("try again another day ${e}")
        }
    };
    let json: Config = match serde_json::from_str(&contents) {
        Ok(success) => {
            println!("parsed json!!!!");
            success
        }
        Err(e) => {
            println!("couldn't parse json");
            panic!("${e}")
        }
    };
    return json;
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub token: String,
}
impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}
