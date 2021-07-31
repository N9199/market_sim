pub mod sql;
pub mod stdio;
use crate::simulation::sim::Company;

use std::collections::BTreeMap;

sum_type! {
    pub enum Error{
        SQL(rusqlite::Error),
        IO(std::io::Error),
    }
}

pub trait IO {
    // Writes info to stdout, where each element of info is a company id with it's price.
    fn write(&mut self, info: &Vec<(usize, f64)>) -> Result<(), Error>;
    fn save(&mut self, companies: &BTreeMap<usize, Company>) -> Result<(), Error>;
    fn load(&mut self, file: &str) -> Result<BTreeMap<usize, Company>, Error>;
}

pub struct EmptyIO {}
impl EmptyIO {
    pub fn new() -> Self {
        EmptyIO {}
    }
}
impl IO for EmptyIO {
    fn write(&mut self, info: &Vec<(usize, f64)>) -> Result<(), Error> {
        Ok(())
    }
    fn save(&mut self, companies: &BTreeMap<usize, Company>) -> Result<(), Error> {
        Ok(())
    }
    fn load(&mut self, file: &str) -> Result<BTreeMap<usize, Company>, Error> {
        Ok(BTreeMap::new())
    }
}
