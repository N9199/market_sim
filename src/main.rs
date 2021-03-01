#[macro_use]
extern crate sum_type;

mod simulation;
mod io;

use rand::distributions::{Distribution, Uniform};

fn main() {
    println!("Test");
    let mut companies = Vec::new();
    let mut last_id: usize = 0;
    let mut rng = rand::thread_rng();
    let price = Uniform::new_inclusive(200.0, 1000.0);
    let delta_p = Uniform::new_inclusive(-100.0, 100.0);
    let volatility = Uniform::new_inclusive(0.001, 0.2);
    let alphabet = (b'a'..=b'z') // Start as u8
        .map(|c| c as char) // Convert all to chars
        .filter(|c| c.is_alphabetic()) // Filter only alphabetic chars
        .collect::<Vec<_>>();
    for (i, c) in alphabet.iter().enumerate() {
        companies.push(simulation::sim::Company::new(last_id, &c.to_string()));
        companies[i].set(
            price.sample(&mut rng),
            delta_p.sample(&mut rng),
            volatility.sample(&mut rng),
        );
        //println!("{:?}",companies[i]);
        last_id += 1;
    }
    let id = Uniform::new(0, (last_id - 1) as usize);
    let deps = Uniform::new(0, 4);
    let val = Uniform::new_inclusive(0.005, 0.4);
    for i in 0..companies.len() {
        for _ in 0..deps.sample(&mut rng) {
            let mut j = id.sample(&mut rng);
            if j >= i {
                j += 1;
            }
            companies[i].add_dep(j, val.sample(&mut rng));
        }
    }
    println!(
        "{}",
        companies
            .iter()
            .fold(String::from("time"), |acc, c| format!(
                "{}|{:^9}",
                acc, c.name
            ))
    );
    println!(
        "{}",
        companies
            .iter()
            .fold(String::from(format!("{:>4}", 0)), |acc, c| format!(
                "{}|{:^9.3}",
                acc, c.price
            ))
    );
    for i in 1..100 {
        for c in &mut companies {
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
        for c in &mut companies {
            c.update(Option::None, &mut rng);
        }
        println!(
            "{}",
            companies
                .iter()
                .fold(String::from(format!("{:>4}", i)), |acc, c| format!(
                    "{}|{:^9.3}",
                    acc, c.price
                ))
        );
    }
}
