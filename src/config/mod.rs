use std::fs;
use std::io::Error as IoError;
use serde::{ Serialize, Deserialize };
use toml;


#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlDatabase {
    username: Option<String>,
    password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigTomlOther {
    api_key: Option<String>,
    secret_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigToml {
    database:       Option<ConfigTomlDatabase>,
    Other:          Option<ConfigTomlOther>,
}


#[derive(Debug)]
pub struct Config {
   username:        String,
   password:        String,
   api_key:         String,
   secret_token:    String
}

impl Config {
    pub fn new () -> Self {
        let config_filepaths : [&str; 2] =
            [ "./cfg/config.toml",
              "./cfg/temp.toml",
            ];

        let mut content: String = "".to_owned();

        for filepath in config_filepaths {
            let result: Result<String, IoError> = fs::read_to_string(filepath);
        
            if (result.is_ok()) {
                content = result.unwrap();
                break;
            }
        }

        let config_toml: ConfigToml = toml::from_str(&content).unwrap_or_else(|_| {
            println!("Failed to create ConfigToml Object out of config file.");
            ConfigToml{
                database: None,
                Other: None,
            }
        });

        let (username, password): (String, String) = match config_toml.database {
            Some(database) => {
                let db_username: String = database.username.unwrap_or_else(|| {
                    println!("Missing field username in table database.");
                    "unknown".to_owned()
                });

                 let db_password: String = database.password.unwrap_or_else(|| {
                    println!("Missing field username in table database.");
                    "unknown".to_owned()
                });

                (db_username, db_password)
            },

            None => {
                println!("Missing table database.");
                ("unknown".to_owned(), "unknown".to_owned())
            }
        };

            let (api_key, secret_token): (String, String) = match config_toml.Other {
            Some(Other) => {
                let db_api_key: String = Other.api_key.unwrap_or_else(|| {
                    println!("Missing field api_key in table database.");
                    "unknown".to_owned()
                });

                 let db_secret_token: String = Other.secret_token.unwrap_or_else(|| {
                    println!("Missing field secret_token in table database.");
                    "unknown".to_owned()
                });

                (db_api_key, db_secret_token)
            },

            None => {
                println!("Missing table Other.");
                ("unknown".to_owned(), "unknown".to_owned())
            }
        };

        /*
        let api_key: String = match config_toml.Other {
            Some(Other) => Other.api_key.unwrap_or_else(|| {
                println!("Missing field api_key in table Other.");
                "unknown".to_owned()
            }),

            None => {
                println!("Missing table other.");
                "unknown".to_owned()
            }
        };

        let secret_token: String = match config_toml.Other {
            Some(Other) => Other.secret_token.unwrap_or_else(|| {
                println!("Missing field secret_token in table Other.");
                "unknown".to_owned()
            }),

            None => {
                println!("Missing table other.");
                "unknown".to_owned()
            }
        };
        */

        Config {
            username:       username,
            password:        password,
            api_key:            api_key,
            secret_token:  secret_token,
        }
    }
}
