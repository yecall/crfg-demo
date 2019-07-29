//extern crate crossbeam;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod message;
mod bft_instance;
mod hashing;

use std::env;
use std::thread;
use std::time::Duration;
use ed25519_dalek::Keypair;

use crossbeam::queue::{ArrayQueue, PushError};

use crate::hashing::blake2_256;
use crate::message::Message;
use crate::message::MessageEnv;
use crate::bft_instance::BftInstance;


pub type Hash = primitive_types::H256;

fn main(){
    let args: Vec<String> = env::args().collect();
    let bft_msg = MessageEnv::new();
    let instance = BftInstance::new();

    println!("bft_msg = {:#?}", bft_msg);
    println!("instance = {:#?}", instance);

    //hash practise
    let s = String::from("werqweqwe");
    let h = blake2_256(s.as_bytes());
    println!("hash = {:?}", hex::encode(h));

    let q = ArrayQueue::new(2);
    assert_eq!(q.push('a'), Ok(()));
}
