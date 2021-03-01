use std::fs;

use crate::io::{Error, IO};
use crate::simulation::sim::Company;

pub struct io {}

impl IO for io {
    fn write(&mut self, info: &Vec<(usize, f64)>) -> Result<(), Error> {
        Ok(())
    }
    fn save(&mut self, companies: &Vec<Company>) -> Result<(), Error> {
        Ok(())
    }
    fn load(&mut self, file: &str) -> Result<Vec<Company>, Error> {
        let info = fs::read_to_string(file)?;
        Ok(Vec::new())
    }
}
