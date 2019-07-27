use std::collections::HashMap;
use crate::bft_instance::{Index, BftInstance};

#[derive(Debug)]
struct Bft{
    is_validator: bool,
    private_key: String,
    replica_id: u8,
    view_number: i64,
    sequence: i64,
    last_exe_seq: i64,
    validators: HashMap<String, u8>,
    instances: HashMap<Index, BftInstance>,
}

impl Bft{
    fn send() -> bool {
        true
    }

    fn recv() -> bool {
        true
    }

    fn new_pre_prepare(proposal: str){
    }
}

