use crate::io::IO;

use std::collections::BTreeMap;
use std::mem;

use log::{debug, info, warn};
use rand::distributions::{Distribution, Uniform};
use rayon::prelude::*;

const eps: f64 = 0.001;

#[derive(Default, Debug, Clone)]
pub struct Company {
    pub id: usize, // Unique ID
    pub name: String,
    pub price: f64, // Number in [0,infty]
    pub delta_p: f64,
    volatility: f64,                     // Number in [0,1]
    pub dependencies: Vec<(usize, f64)>, // List of Companies on which this company depends on, and how much it depends on it (value in [-1,-eps)U(eps,1] with eps essentially zero)
    bankrupt: bool,
}

impl Company {
    pub fn new(id: usize, name: &str) -> Company {
        Company {
            id: id,
            name: name.to_string(),
            price: 0.0,
            delta_p: 0.0,
            volatility: 0.0,
            dependencies: Vec::new(),
            bankrupt: false,
        }
    }

    pub fn load(
        id: usize,
        name: String,
        price: f64,
        delta_p: f64,
        volatility: f64,
        bankrupt: bool,
        dependencies: Vec<(usize, f64)>,
    ) -> Company {
        Company {
            id,
            name: name.to_string(),
            price,
            delta_p,
            volatility,
            dependencies,
            bankrupt,
        }
    }

    pub fn set(&mut self, price: f64, delta_p: f64, volatility: f64) {
        self.price = price;
        self.delta_p = delta_p;
        self.volatility = volatility;
    }

    pub fn add_dep(&mut self, dep: usize, val: f64) {
        if val.abs() > eps {
            self.dependencies.push((dep, val));
        }
    }

    pub fn update(&self, companies: &BTreeMap<usize, Company>, effect: Option<(f64, f64)>) -> Self {
        let mut rng = rand::thread_rng();
        let mut new = self.clone();
        let temp = effect.unwrap_or((1.0, 1.0));
        let temp1 = Uniform::new_inclusive(temp.0, temp.1);
        let temp2 = Uniform::new_inclusive(-self.volatility, self.volatility);
        new.price *= 1.0 + temp2.sample(&mut rng);

        new.price += self
            .dependencies
            .par_iter()
            .map(|(id, v)| companies[id].delta_p * v)
            .sum::<f64>(); //Essentially matrix multiplication, should be fully parallelizable with gpu (for now use rayon)

        if new.price <= 0.0 {
            new.price = 0.0;
        }
        new.price *= temp1.sample(&mut rng);
        new.delta_p = new.price - self.price;
        new
    }
}

pub fn sim<T: IO>(
    active: &Vec<usize>,
    companies: &mut BTreeMap<usize, Company>,
    runs: u32,
    out: T,
) {
    let mut companies2: BTreeMap<usize, Company> = BTreeMap::new();
    for i in 0..runs {
        for id in active {
            // Should be fully parallelizable
            companies2.insert(*id, companies[id].update(&companies, None));
        }
        mem::swap(companies, &mut companies2); //This inherently "deletes" inactive companies
        println!(
            "{}",
            companies
                .iter()
                .fold(String::from(format!("{:>4}", i)), |acc, c| format!(
                    "{}|{:^9.3}",
                    acc, c.1.price
                ))
        );
    }
}
