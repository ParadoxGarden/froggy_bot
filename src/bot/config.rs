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

    let mut user_file_path: &str = ""; // user defined

    // 2
    match env::var(env_config) {
        Ok(user_file_path) => println!("{env_config}: {user_file_path:?}"),
        Err(_e) => {
            println!(
                "couldn't read '${env_config}'! trying default config at {default_config_file}"
            ); // test making this obscenely long so it will fmt on save
            user_file_path = default_config_file;
        }
    };
    //precedence?? maybe set boolean
    let contents: &str = &fs::read_to_string(user_file_path).unwrap();
    let json: Config = serde_json::from_str(contents).unwrap();
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
