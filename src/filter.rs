use serde::{Deserialize, Serialize};
use xorf::{BinaryFuse16, BinaryFuse32, BinaryFuse8, Filter as FuseFilter};

#[derive(Copy,Clone)]
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
        if let Err(_) = filter {
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
}
