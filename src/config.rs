use bitvec::prelude::*;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{
    ffi::OsString,
    io::{Read, Write},
    path::PathBuf,
};

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub password_filter: Option<OsString>,
    pub resume_token: Option<Resume>,
}

#[derive(Serialize, Deserialize)]
pub struct Resume {
    pub resume: BitBox,
    pub download_file: OsString,
}

impl Config {
    pub fn load() -> Self {
        let config_file = Self::get_config_file();
        let mut file = std::fs::File::options()
            .read(true)
            .write(true)
            .create(true)
            .open(config_file)
            .expect("Unable to open config file");

        let mut file_contents = Vec::new();
        file.read_to_end(&mut file_contents).expect("Unable to read config file");
        let result: Result<Self, _> = bincode::deserialize(&file_contents);
        match result {
            Ok(result) => result,
            Err(_) => Config {
                ..Default::default()
            },
        }
    }

    pub fn store(self) {
        let config_file = Self::get_config_file();
        let mut file = std::fs::File::options()
            .write(true)
            .truncate(true)
            .create(true)
            .open(config_file)
            .expect("Unable to open config file");
        let serialized = bincode::serialize(&self).expect("Unable to create configuration");
        file.write_all(&serialized).expect("Unable to write configuration file");
    }

    fn get_config_file() -> PathBuf {
        let mut result = ProjectDirs::from("rs", "", "haveibeenpwned")
            .unwrap()
            .config_dir()
            .to_owned();
        std::fs::create_dir_all(&result).unwrap();
        result.push("config");
        result
    }
}
