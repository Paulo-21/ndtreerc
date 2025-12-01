use std::io::{Write, stdout};

use rand::prelude::*;

mod ffi;
mod ndtree;

fn main() {
    let x = [0, 2, 99, 3];

    const nbcrit: i32 = 8;
    let mut obj1 = [0; nbcrit as usize];
    let mut rng = rand::rng();
    let r = ndtree::NdsTree::new(nbcrit, 4, nbcrit + 1, 20);
    for k in 1..10_000_000 {
        /*let x: [i32; 4] = [
            rng.random::<bool>() as i32,
            rng.random::<bool>() as i32,
            rng.random::<bool>() as i32,
            rng.random::<bool>() as i32,
        ];*/
        for cr in obj1.iter_mut() {
            *cr = rng.random::<i32>().abs()
        }

        //print!("\r{k}");
        //println!("{:?}", obj1);
        r.add(&x, &obj1);
        let _ = stdout().flush();
    }
}
/*
let obj1 = [
    rng.random::<i32>().abs(),
    rng.random::<i32>().abs(),
    rng.random::<i32>().abs(),
    rng.random::<i32>().abs(),
];*/
