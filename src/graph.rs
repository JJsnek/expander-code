use rand::Rng;

pub fn generate_random_expander(n: usize, m:usize, d:usize) -> Vec<Vec<usize>> {

    let mut rng  =rand::thread_rng();
    let mut graph =vec![vec![];m];

    for left in 0..n{
        for _ in 0..d{
            let right=rng.gen_range(0..m);
            graph[right].push(left);
        }
    }
    graph
}