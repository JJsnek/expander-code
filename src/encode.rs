use ark_bls12_381::Fr;
use ark_ff::Zero;


pub fn encode(
    message: &Vec<Fr>,
    graph: &Vec<Vec<usize>>,
)->Vec<Fr>{

    let m=graph.len();
    //initialize codeword with zeros
    let mut codeword=vec![Fr::zero();m];//m komada
    
    //for each right node
    for j in 0..m{
        let neighbors=&graph[j];

        let mut sum=Fr::zero();
        for &i in neighbors{
            sum+=message[i];
        }
        codeword[j]=sum;
    }
    codeword
}


//weighted

pub fn encodew(
    message: &Vec<Fr>,
    graph: &Vec<Vec<(usize, Fr)>>
) -> Vec<Fr> {
    let m = graph.len();

    let mut codeword = vec![Fr::zero(); m];

    for j in 0..m {
        let neighbors = &graph[j];

        let mut sum = Fr::zero();

        for &(i, coeff) in neighbors {
            sum += coeff * message[i];
        }

        codeword[j] = sum;
    }

    codeword
}