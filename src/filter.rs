use std::{
    hash::{Hash, Hasher},
    io::{BufReader, Write},
};

use crate::password::{self, Password};
use roaring::bitmap::RoaringBitmap;
use serde::{Deserialize, Serialize};
use siphasher::sip::SipHasher13;
use std::ffi::OsString;
use xorf::{BinaryFuse16, BinaryFuse32, BinaryFuse8, Filter as FuseFilter};

const RB_SIZE: f64 = 4.0 * 8.0 * 1024.0 * 1024.0 * 1024.0;

#[derive(Copy, Clone)]
pub enum FilterSize {
    Small,
    Medium,
    Large,
}

pub struct RB(u64, RoaringBitmap);

impl RB {
    pub fn new(size: u64) -> Self {
        let k = f64::round(RB_SIZE / size as f64 * f64::ln(2.0));
        Self(k as u64, RoaringBitmap::new())
    }

    pub fn insert(&mut self, password: &str) {
        let first_hash = password::hash(password);
        for i in 0..self.0 {
            let mut second_hash = SipHasher13::new_with_keys(i, i);
            first_hash.hash(&mut second_hash);
            let final_hash = second_hash.finish() as u32;
            self.1.insert(final_hash);
        }
    }

    pub fn check(&self, password: &str) -> bool {
        let mut result = true;
        let first_hash = password::hash(password);
        for i in 0..self.0 {
            let mut second_hash = SipHasher13::new_with_keys(i, i);
            first_hash.hash(&mut second_hash);
            let final_hash = second_hash.finish() as u32;
            result &= self.1.contains(final_hash);
        }
        result
    }

    pub fn serialize(&self, mut writer: impl Write) -> std::io::Result<()> {
        writer.write_all(&mut self.0.to_le_bytes()).unwrap();
        self.1.serialize_into(writer)
    }

    pub fn len(&self) -> u64 {
        self.1.len()
    }

    pub fn open(file: OsString) -> Result<Self, ()> {
        let input_file = std::fs::File::options().read(true).open(file);
        if let Err(error) = input_file {
            eprintln!("Unable to open the input file: {}", error.kind());
            return Err(());
        }

        let mut input_file = BufReader::new(input_file.unwrap());

        let mut wow = [0; 8];
        std::io::Read::read_exact(&mut input_file, &mut wow);

        let hash_count: u64 = u64::from_le_bytes(wow);

        let filter = RoaringBitmap::deserialize_from(input_file);

        if filter.is_err() {
            eprintln!("Input file is not a valid filter");
            return Err(());
        }

        let filter = filter.unwrap();

        Ok(Self(hash_count, filter))
    }
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

    pub fn open_filter(file: OsString) -> Option<Self> {
        let input_file = std::fs::File::options().read(true).open(file);
        if let Err(error) = input_file {
            eprintln!("Unable to open the input file: {}", error.kind());
            return None;
        }

        let input_file = BufReader::new(input_file.unwrap());

        let filter: Result<Self, _> = bincode::deserialize_from(input_file);

        if filter.is_err() {
            eprintln!("Input file is not a valid filter");
            return None;
        }

        Some(filter.unwrap())
    }
}
