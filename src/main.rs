use std::{
    io::{Write, stdout},
    time::Instant,
};

use rand::prelude::*;

use crate::list_pareto::ParetoList;

mod ffi;
mod list_pareto;
mod ndtree;

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    let x = [0, 2, 99, 3];

    const nbcrit: i32 = 8;
    let mut obj1 = [0; nbcrit as usize];
    let mut rng = rand::rng();
    let start = Instant::now();
    let r = ndtree::NdsTree::new(nbcrit, 4, nbcrit + 1, 20);
    for k in 1..100000 {
        for cr in obj1.iter_mut() {
            *cr = rng.random::<i32>().abs()
        }
        r.add(&x, &obj1);
        let _ = stdout().flush();
    }
    println!("ND-Tree : {}", start.elapsed().as_millis());
    let start = Instant::now();
    let mut list_pareto = ParetoList::new();
    for k in 1..100000 {
        for cr in obj1.iter_mut() {
            *cr = rng.random::<i32>().abs()
        }
        list_pareto.insert(obj1);
        let _ = stdout().flush();
    }
    println!("LIST : {}", start.elapsed().as_millis());
}
/*
let obj1 = [
    rng.random::<i32>().abs(),
    rng.random::<i32>().abs(),
    rng.random::<i32>().abs(),
    rng.random::<i32>().abs(),
];*/
