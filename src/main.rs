mod graph;
mod encode;

use ark_bls12_381::Fr;
use ark_ff::{UniformRand};
use rand::thread_rng;

use graph::generate_random_expander;
use encode::encode;

fn main() {
    println!("Hello, world!");

    let mut rng=thread_rng();
    let a=Fr::rand(&mut rng);
    let b=Fr::rand(&mut rng);

    let c=a+b;

    println!("a + b = {:?}",c);

    let n=5; let m=20; let d=3;

    let message:Vec<Fr>=(0..n).map(|_|Fr::rand(&mut rng)).collect();
    println!("Message {:?}", message);

    let graph=generate_random_expander(n,m,d);
    println!("{:?}", graph);

    let codeword=encode(&message, &graph);
    println!("Codeword: {:?}", codeword);

}
