//extern crate crossbeam;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate rand;
extern crate ed25519_dalek;

mod message;
mod bft_instance;
mod hashing;

use std::env;
use std::thread;
use std::time::Duration;

use crossbeam::queue::{ArrayQueue, PushError};

use rand::Rng;
use rand::rngs::OsRng;
use ed25519_dalek::Keypair;
use ed25519_dalek::Signature;

use crate::hashing::blake2_256;
use crate::message::BftMsg;
use crate::message::BftMsgPkg;
use crate::bft_instance::BftInstance;

pub type Hash = primitive_types::H256;

fn main(){
    let args: Vec<String> = env::args().collect();
    let bft_msg =BftMsgPkg::new();
    let instance = BftInstance::new();

    println!("bft_msg = {:#?}", bft_msg);
    println!("instance = {:#?}", instance);

    //hash practise
    let s = String::from("werqweqwe");
    let h = blake2_256(s.as_bytes());
    println!("hash = {:?}", hex::encode(h));

    let mut csprng: OsRng = OsRng::new().unwrap();
    let keypair: Keypair = Keypair::generate(&mut csprng);
    let signed_data= keypair.sign(s.as_bytes());
    let ret = keypair.verify(s.as_bytes(),&signed_data);
    match ret {
        Ok(value) => println!("verify succeed: {:?}", value),
        Err(error) => println!("verify error: {}", error)
    }
    
    let q = ArrayQueue::new(2);
    assert_eq!(q.push('a'), Ok(()));
}
