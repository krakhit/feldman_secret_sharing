pub mod feldman_secret_gen;
use ark_ff::{Field,Zero,One, BigInteger256};
use ark_ec::{group::Group};
use ark_bls12_381::{Fr as F, G1Projective as G};
use ark_std::{UniformRand};
use feldman_secret_gen::*;
use num_bigint::{BigUint};
use sha2::{Sha256,Digest};

pub fn main(){
    //regular proof
    //for user id x_i =F::from(13), and secret share poly(x_i)
    let share_id_user = F::new(BigInteger256([120259084260, 15510977298029211676, 7326335280343703402, 5909200893219589146]));
    let secret_share_user =F::new(BigInteger256([17556863090070465933, 17483734390907017496, 11275179645852245857, 1659062075969486508]));
    //user verifies his share
    verify(share_id_user, secret_share_user);
}
