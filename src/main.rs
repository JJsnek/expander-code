mod graph;
mod encode;

use ark_bls12_381::Fr;
use ark_ff::{UniformRand};
use rand::thread_rng;
use graph::generate_random_expander;

fn main() {
    println!("Hello, world!");
    let mut rng=thread_rng();
    let a=Fr::rand(&mut rng);
    let b=Fr::rand(&mut rng);

    let c=a+b;

    println!("a + b = {:?}",c);

    let graph=generate_random_expander(10,20,3);
    println!("{:?}", graph);

}
