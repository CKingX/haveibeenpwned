use std::hash::{Hash, Hasher};

use crate::password::{self, Password};
use serde::{Deserialize, Serialize};
use siphasher::sip::SipHasher13;
use xorf::{BinaryFuse16, BinaryFuse32, BinaryFuse8, Filter as FuseFilter};

#[derive(Copy, Clone)]
pub enum FilterSize {
    Small,
    Medium,
    Large,
}

#[derive(Serialize, Deserialize)]
pub enum FilterType {
    Small(BinaryFuse8),
    Medium(BinaryFuse16),
    Large(BinaryFuse32),
}

impl FilterType {
    fn new(items: &[u64], filter_size: FilterSize) -> Result<Self, &str> {
        match filter_size {
            FilterSize::Small => Ok(FilterType::Small(BinaryFuse8::try_from(items)?)),
            FilterSize::Medium => Ok(FilterType::Medium(BinaryFuse16::try_from(items)?)),
            FilterSize::Large => Ok(FilterType::Large(BinaryFuse32::try_from(items)?)),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Filter {
    len: u64,
    keys: (u64, u64),
    filter: FilterType,
}

impl Filter {
    pub fn new(items: &[u64], keys: (u64, u64), filter_size: FilterSize) -> Result<Self, ()> {
        let filter = FilterType::new(items, filter_size);
        if filter.is_err() {
            return Err(());
        }

        let filter = filter.unwrap();

        Ok(Filter {
            len: u64::try_from(items.len()).unwrap(),
            keys,
            filter,
        })
    }

    pub fn len(&self) -> u64 {
        self.len
    }

    pub fn check_password(&self, password: &str) -> Password {
        let first_hash = password::hash(password);
        let mut second_hash = SipHasher13::new_with_keys(self.keys.0, self.keys.1);
        first_hash.hash(&mut second_hash);
        let final_hash = second_hash.finish();

        let result = match &self.filter {
            FilterType::Small(filter) => filter.contains(&final_hash),
            FilterType::Medium(filter) => filter.contains(&final_hash),
            FilterType::Large(filter) => filter.contains(&final_hash),
        };

        match result {
            true => Password::CompromisedPassword,
            false => Password::SafePassword,
        }
    }
}
