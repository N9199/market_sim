#[macro_use]
extern crate sum_type;
extern crate itertools;

mod io;
mod simulation;

use io::EmptyIO;
use simulation::sim::{sim, Company};

use std::collections::BTreeMap;

use itertools::iproduct;
use rand::distributions::{Distribution, Uniform};

fn main() {
    println!("Test");
    let mut companies = BTreeMap::new();
    let mut last_id: usize = 0;
    let mut rng = rand::thread_rng();
    let price = Uniform::new_inclusive(200.0, 1000.0);
    let delta_p = Uniform::new_inclusive(-100.0, 100.0);
    let volatility = Uniform::new_inclusive(0.001, 0.2);
    let alphabet = (b'a'..=b'z') // Start as u8
        .map(|c| c as char) // Convert all to chars
        .filter(|c| c.is_alphabetic()) // Filter only alphabetic chars
        .collect::<Vec<_>>();
    let temp = iproduct!(alphabet.clone(), alphabet.clone(), alphabet.clone());
    for (i, c) in temp.enumerate() {
        companies.insert(
            i,
            simulation::sim::Company::new(last_id, &format!("{}{}{}", c.0, c.1, c.2)),
        );
        companies.get_mut(&i).unwrap().set(
            price.sample(&mut rng),
            delta_p.sample(&mut rng),
            volatility.sample(&mut rng),
        );
        //println!("{:?}",companies[i]);
        last_id += 1;
    }
    let active: Vec<usize> = (0..companies.len()).collect();
    let id = Uniform::new(0, (last_id - 1) as usize);
    let deps = Uniform::new(0, 4);
    let val = Uniform::new_inclusive(0.005, 0.4);
    for i in 0..companies.len() {
        for _ in 0..deps.sample(&mut rng) {
            let mut j = id.sample(&mut rng);
            if j >= i {
                j += 1;
            }
            companies
                .get_mut(&i)
                .unwrap()
                .add_dep(j, val.sample(&mut rng));
        }
    }
    println!(
        "{}",
        companies
            .iter()
            .fold(String::from("time"), |acc, c| format!(
                "{}|{:^9}",
                acc, c.1.name
            ))
    );
    println!(
        "{}",
        companies
            .iter()
            .fold(String::from(format!("{:>4}", 0)), |acc, c| format!(
                "{}|{:^9.3}",
                acc, c.1.price
            ))
    );

    sim(&active, &mut companies, 1000, EmptyIO::new());
}
