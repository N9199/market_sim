use crate::io::{Error, IO};
use crate::simulation::sim::Company;

use std::collections::BTreeMap;
use std::fs;

pub struct stdIO {}

impl IO for stdIO {
    fn write(&mut self, info: &Vec<(usize, f64)>) -> Result<(), Error> {
        Ok(())
    }
    fn save(&mut self, companies: &BTreeMap<usize, Company>) -> Result<(), Error> {
        Ok(())
    }
    fn load(&mut self, file: &str) -> Result<BTreeMap<usize, Company>, Error> {
        let info = fs::read_to_string(file)?;
        Ok(BTreeMap::new())
    }
}
