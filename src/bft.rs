use std::collections::HashMap;
use crate::message::*;
use crate::bft_instance::*;

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

    fn recv(msg: &str) -> bool {
        let pkg = BftMsgPkg::deserialize(msg);
        match {
         
        }
    }

    fn get_replica_id_by_address(addr: &str){
        //Bft.validators[addr]
    }

    fn replica_id_check(){
        //public key ==> address
        //replica id = validators[address]
        //check if id == msg.replica_id
    }

    fn broadcast_pre_prepare(){ }

    fn broadcast_prepare(){ }

    fn broadcast_commit(){ }

    fn broadcast_view_change(){ }

    fn broadcast_new_view(){}

    fn process_pre_prepare(){}

    fn process_prepare(){}

    fn process_commit(){}

    fn process_view_change(){}

    fn process_new_view(){}
}

