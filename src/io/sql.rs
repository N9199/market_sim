use crate::io::{Error, IO};
use crate::simulation::sim::Company;

use std::collections::BTreeMap;

use rusqlite::{params, Connection};

pub struct sqlIO {
    db: Connection,
}

impl IO for sqlIO {
    fn write(&mut self, info: &Vec<(usize, f64)>) -> Result<(), Error> {
        for (id, price) in info {
            let mut temp = self
                .db
                .prepare(&format!("SELECT MAX(time) FROM {}_History", id))?;
            let temp = temp.query_map([], |row| row.get(0))?;
            let time: usize = temp.last().unwrap_or(Ok(0))?;
        }
        Ok(())
    }
    fn save(&mut self, companies: &BTreeMap<usize, Company>) -> Result<(), Error> {
        Ok(())
    }
    fn load(&mut self, file: &str) -> Result<BTreeMap<usize, Company>, Error> {
        self.db = Connection::open(file)?;
        let mut temp = self
            .db
            .prepare("SELECT id, name, price, delta_p, volatility, bankrupt FROM Companies")?;
        let temp = temp.query_map(params![], |row| {
            Ok(Company::load(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
                Vec::new(),
            ))
        })?;
        let mut out = BTreeMap::new();
        for c in temp {
            if let Ok(c) = c {
                out.insert(c.id, c);
            }
        }
        for (id, c) in &mut out {
            let mut temp = self.db.prepare(&format!(
                "SELECT in_id, out_id, weight, FROM Dependencies WHERE in_id = {}",
                id
            ))?;
            let temp = temp.query_map(params![], |row| Ok((row.get(1)?, row.get(2)?)))?;
            for e in temp {
                let t = e.unwrap();
                c.add_dep(t.0, t.1);
            }
        }
        Ok(out)
    }
}
