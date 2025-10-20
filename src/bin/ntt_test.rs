use ark_bn254::Fr;
use ark_ff::BigInt;
use whir::ntt::{interleaved_rs_encode, RSFr};

fn main() {
    let v = vec![Fr::from(BigInt([1; 4])); 1 << 22];
    interleaved_rs_encode::<_, RSFr>(&v, 2, 4);
}
