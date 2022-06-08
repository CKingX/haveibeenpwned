use bitvec::prelude::*;
use serde::{Deserialize, Serialize};
use std::ffi::OsString;

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
        confy::load("haveibeenpwned").unwrap()
    }

    pub fn store(self) {
        confy::store("haveibeenpwned", self).unwrap()
    }
}
