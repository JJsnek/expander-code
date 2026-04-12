use ark_bls12_381::Fr;
use ark_ff::UniformRand;
use rand::thread_rng;
use std::time::Instant;

use crate::graph::generate_weighted_expander;
use crate::encode::encodew;

use std::fs::File;
use std::io::Write;

pub fn run_benchmarks(){
    let mut rng=thread_rng();
    
    //params to test
    let ns=vec![100,500,1000,5000];
    let ds=vec![3,5,10,15];

        let mut file=File::create("benchmark_res.csv").unwrap();
        writeln!(file,"n,d,time_ms,time_per_element_us").unwrap();

    println!("n,d,time_ms,time_per_element_us");

    for &n in &ns{
        for &d in &ds{
            let m=2*n; //oke

            //gen rnd message
            let message: Vec<Fr>=(0..n).map(|_|Fr::rand(&mut rng)).collect();

            //gen g chan
            let graph=generate_weighted_expander(n,m,d);
            
            let start=Instant::now();
            let _codeword = encodew(&message,&graph);
            let duration=start.elapsed();
            let time_ms=duration.as_secs_f64()*1000.0;
            let time_per_element_us= (time_ms*1000.0)/n as f64;

            //println!("{},{},{:.4},{:.4}",n,d,time_ms,time_per_element_us);
            println!("n = {:<6} d = {:<3} | time = {:>8.4} ms | per element = {:>8.4} µs",n, d, time_ms, time_per_element_us);
                writeln!(file,"{},{},{:.4},{:.4}",n,d,time_ms,time_per_element_us).unwrap();
        }
    }

    

}