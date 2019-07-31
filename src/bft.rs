use std::collections::HashMap;
use crate::message::*;
use crate::bft_instance::*;
use ed25519_dalek::Signature;
use ed25519_dalek::PublicKey;

#[derive(Debug)]
pub struct Bft{
    is_validator: bool,
    private_key: String,
    replica_id: i8,
    view_number: i64,
    sequence: i64,
    last_exe_seq: i64,
    validators: HashMap<String, i8>,
    instances: HashMap<Index, BftInstance>,
}

impl Bft{
    fn send() -> bool {
        true
    }

    fn process(&mut self, msg: &str) -> bool {
        let pkg = BftMsgPkg::deserialize(msg);
        if !Self::signature_valid (&pkg){return false;}

        match pkg.message() {
            BftMsg::PrePrePare(msg) =>{
                self.replica_id_valid(&pkg.signature().public_key, msg.replica_id);
                self.process_pre_prepare(pkg);
            },
            BftMsg::PrePare(msg) => {
                self.replica_id_valid(&pkg.signature().public_key, msg.replica_id);
                self.process_prepare(pkg);
            },
            BftMsg::Commit(msg) => {
                self.replica_id_valid(&pkg.signature().public_key, msg.replica_id);
                self.process_commit(pkg);
            },
            BftMsg::ViewChange(msg) => {
                let view_change_basic = msg.view_change.message();
                match msg.view_change.message() {
                    BftMsg::ViewChangeBasic(basic) =>{
                        self.replica_id_valid(&pkg.signature().public_key, basic.replica_id);
                    },
                    _ => (),
                };
                self.process_view_change(pkg);
            },
            BftMsg::NewView(msg) => {
                self.replica_id_valid(&pkg.signature().public_key, msg.replica_id);
                self.process_new_view(pkg);
            },
            _ => println!("Undefined bft message type.")
        }
        true
    }

    fn proposal_valid(proposal: &str) -> bool {
        //check block size (in limit)
        //check if transaction is legal
        true
    }

    fn set_bft_instance(msg_pkg: &BftMsgPkg){
        //create Index
        //search instance by Index
        //if instance not found, create, and insert to instances
        //if(some instance's sequence == BftMsgPkg's sequence && view number < BftMsgPkg's view numer)
        //  delete the instance
        //  insert BftMsgPkg into instance
    }

    fn public_key2addr(pub_key: &str) -> String {
        //STAKE FUNC, shoud re-complement
        String::from(pub_key)
    }

    fn signature_valid(msg_pkg: &BftMsgPkg) -> bool {
        //STAKE FUNC, shoud re-complement
        //let sig = msg_pkg.signature();
        //sig.public_key.verify(msg_pkg.message().serialize(), sig.sign_data);
        true
    }

    fn replica_id_valid(&self, public_key: &str, replica_id: i8) -> bool {
        //1 verify signature
        //2 public key ==> address
        //  validators[address] ==> replica_id
        //  check if id == msg.replica_id

        match self.validators.get(public_key){
            Some(id) => *id == replica_id,
            None => false,
        }
    }

    fn create_pre_prepare(&mut self){
    }

    fn create_prepare(&mut self){
        //create BftMsgPkg
        //create BftMsg, and set view_number,sequence,proposal_digest,replica_id
        //set signature
    }

    fn create_commit(&mut self){ }

    fn create_view_change(&mut self){ }

    fn send_new_view(&mut self){}

    fn process_pre_prepare(&mut self, msg_pkg: BftMsgPkg){
        //check proposal digest
        //check proposal value
        //update bft instance
        //create prepare msg
        //send prepare msg
    }

    fn process_prepare(&mut self, msg_pkg: BftMsgPkg){
        //check prepare proposal digest == preprepare proposal digest
    }

    fn process_commit(&mut self, msg_pkg: BftMsgPkg){
    }

    fn process_view_change(&mut self, msg_pkg: BftMsgPkg){

    }

    fn process_new_view(&mut self, msg_pkg: BftMsgPkg){
    }
}

