use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;

use crate::io::IO;

#[derive(Default, Debug, Clone)]
pub struct Company {
    pub id: usize, // Unique ID
    pub name: String,
    pub price: f64, // Number in [0,infty]
    new_p: f64,
    pub delta_p: f64, // ! Note: price=last_p+delta_p is only true after calling update
    volatility: f64,  // Number in [0,1]
    pub dependencies: Vec<(usize, f64)>, // List of Companies on which this company depends on, and how much it depends on it (value in [-1,1])
    bankrupt: bool,
}

impl Company {
    pub fn new(id: usize, name: &str) -> Company {
        Company {
            id: id,
            name: name.to_string(),
            price: 0.0,
            new_p: 0.0,
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
        new_p: f64,
        delta_p: f64,
        volatility: f64,
        bankrupt: bool,
        dependencies: Vec<(usize, f64)>,
    ) -> Company {
        Company {
            id,
            name: name.to_string(),
            price,
            new_p,
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
        self.dependencies.push((dep, val));
    }

    pub fn advance(&mut self, rng: &mut ThreadRng) {
        self.new_p = self.price;
        let temp = Uniform::new_inclusive(-self.volatility, self.volatility);

        self.new_p *= 1.0 + temp.sample(rng);
    }

    pub fn dep(&mut self, deps: Vec<(f64, f64)>) {
        for c in &deps {
            self.new_p += c.0 * c.1;
        }
    }
    pub fn update(&mut self, effect: Option<(f64, f64)>, rng: &mut ThreadRng) {
        let temp = effect.unwrap_or((1.0, 1.0));
        let temp = Uniform::new_inclusive(temp.0, temp.1);
        if self.new_p <= 0.0 {
            self.new_p = 0.0;
        }
        self.new_p *= temp.sample(rng);
        self.delta_p = self.new_p - self.price;
        self.price = self.new_p;
    }
}

pub fn sim<T: IO>(companies: &mut Vec<Company>, runs: u32, out: T) {
    let mut rng = rand::thread_rng();
    for _ in 0..runs {
        for c in &mut *companies {
            c.advance(&mut rng);
        }
        for i in 0..companies.len() {
            let temp = companies[i]
                .dependencies
                .iter()
                .map(|(c, v)| (companies[*c].delta_p, *v))
                .collect();
            companies[i].dep(temp);
        }
        for c in &mut *companies {
            c.update(None, &mut rng);
        }
    }
}
