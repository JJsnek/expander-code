use ark_bls12_381::Fr;
use ark_ff::UniformRand;
use rand::{Rng, thread_rng};

//rnd Fr
pub fn generate_weighted_expander(n:usize,m:usize,d:usize)->Vec<Vec<(usize,Fr)>>{
    let mut rng=thread_rng();
    let mut graph=vec![vec![];m];

    for i in 0..n{
        for _ in 0..d{
            let j=rng.gen_range(0..m);
            let coeff=Fr::rand(&mut rng);
            graph[j].push((i,coeff));
        }
    }
    graph
}


//1
pub fn generate_random_expander(n: usize, m:usize, d:usize) -> Vec<Vec<usize>> {

    let mut rng  =rand::thread_rng();
    let mut graph =vec![vec![];m];

    for j in 0..m{
        let i=rng.gen_range(0..n);
        graph[j].push(i);
    }

    for _ in 0..(n*d-m){//od 0
        let i=rng.gen_range(0..n);
        let j= rng.gen_range(0..m);
        graph[j].push(i);
    }

    graph
}

