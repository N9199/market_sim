pub mod sql;
pub mod stdio;
use crate::simulation::sim::Company;
sum_type! {
    pub enum Error{
        sql(rusqlite::Error),
        io(std::io::Error),
    }
}

pub trait IO {
    // Writes info to stdout, where each element of info is a company id with it's price.
    fn write(&mut self, info: &Vec<(usize, f64)>) -> Result<(), Error>;
    fn save(&mut self, companies: &Vec<Company>) -> Result<(), Error>;
    fn load(&mut self, file: &str) -> Result<Vec<Company>, Error>;
}

pub struct empty {}
impl IO for empty {
    fn write(&mut self, info: &Vec<(usize, f64)>) -> Result<(), Error> {
        Ok(())
    }
    fn save(&mut self, companies: &Vec<Company>) -> Result<(), Error> {
        Ok(())
    }
    fn load(&mut self, file: &str) -> Result<Vec<Company>, Error> {
        Ok(Vec::<Company>::new())
    }
}
