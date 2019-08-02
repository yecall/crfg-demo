use crate::message::*;
use crate::bft_instance::*;
use crate::hashing::blake2_256;
use crate::view_change_instance::*;

use ed25519_dalek::Signature;
use ed25519_dalek::PublicKey;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Bft{
    pub is_validator: bool,
    pub private_key: String,
    pub replica_id: i8,
    pub view_number: i64,
    pub view_active: bool,
    pub sequence: i64,
    pub last_exe_seq: i64,
    pub validators: HashMap<String, i8>,
    pub instances: HashMap<BftInstanceIndex, BftInstance>,
    pub ckp_interval: i64,
    pub low_water_mark: i64,

    pub vc_instances: HashMap<ViewChangeIndex, ViewChangeInstance>,
    pub last_check_time: i64,
}

impl Bft{
    pub fn new() -> Bft {
        Bft{
            is_validator: false,
            private_key: String::new(),
            replica_id: -1,
            view_number: 0,
            view_active: true,
            sequence: 0,
            last_exe_seq: 0,
            validators: HashMap::new(),
            instances: HashMap::new(),
            ckp_interval: 5,
            low_water_mark: 0,
            vc_instances: HashMap::new(),
            last_check_time: 0,
        }
    }

    fn send() -> bool {
        true
    }

    fn process(&mut self, msg: &str) -> bool {
        let pkg = BftMsgPkg::deserialize(msg);
        if !Self::signature_valid (&pkg){return false;}

        match pkg.message() {
            BftMsg::PrePrePare(msg) =>{
                self.process_pre_prepare(msg.clone(), pkg);
            },
            BftMsg::PrePare(msg) => {
                self.process_prepare(msg.clone(), pkg);
            },
            BftMsg::Commit(msg) => {
                self.process_commit(msg.clone(), pkg);
            },
            BftMsg::ViewChange(msg) => {
                let view_change_basic = msg.view_change.message();
                match msg.view_change.message() {
                    BftMsg::ViewChangeBasic(basic) =>{
                        self.process_view_change(msg.clone(), basic.clone(), pkg);
                    },
                    _ => (),
                };
            },
            BftMsg::NewView(msg) => {
                self.process_new_view(msg.clone(), pkg);
            },
            _ => println!("Undefined bft message type.")
        }
        true
    }

    fn proposal_valid(proposal: &[u8]) -> bool {
        //check block header legality
        true
    }

    fn create_bft_instance(&mut self, msg: PrePrePare, msg_pkg: BftMsgPkg){
        let index = BftInstanceIndex {
            sequence: msg.sequence,
            view_number: msg.view_number
        };

        let instance = BftInstance{
            phase: BftMsgType::BftPrePrePared,
            pre_prepare: msg,
            prepares: HashMap::new(),
            commits: HashMap::new(),
            pre_prepare_pkg: msg_pkg,
            prepares_pkgs: vec![],
            commits_pkgs: vec![],
        };

        self.instances.insert(index, instance);

        //delete useless instance
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

    fn digest_valid(data: &[u8], digest: &[u8]) -> bool {
        if blake2_256(data) == digest {
            true
        }
        else{
            false
        }
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

    fn process_pre_prepare(&mut self, msg: PrePrePare, msg_pkg: BftMsgPkg) -> bool {
        self.replica_id_valid(&msg_pkg.signature().public_key, msg.replica_id);

        let proposal = BftMsg::serialize(msg_pkg.message());
        if !Self::digest_valid(proposal.as_bytes(), msg.proposal_hash.as_bytes()) {
            return false;
        }

        if !Self::proposal_valid(proposal.as_bytes()) {
            return false;
        }

        self.create_bft_instance(msg, msg_pkg);


        //create prepare msg
        //send prepare msg
        true
    }

    fn process_prepare(&mut self, msg: PrePare, msg_pkg: BftMsgPkg){
        self.replica_id_valid(&msg_pkg.signature().public_key, msg.replica_id);
        //check prepare proposal digest == preprepare proposal digest
    }

    fn process_commit(&mut self, msg: Commit, msg_pkg: BftMsgPkg){
        self.replica_id_valid(&msg_pkg.signature().public_key, msg.replica_id);
    }

    fn process_view_change(&mut self, msg: ViewChange, basic: ViewChangeBasic, msg_pkg: BftMsgPkg){
        self.replica_id_valid(&msg_pkg.signature().public_key, basic.replica_id);

    }

    fn process_new_view(&mut self, msg: NewView, msg_pkg: BftMsgPkg){
        self.replica_id_valid(&msg_pkg.signature().public_key, msg.replica_id);
    }

    fn update_validators(&mut self, validators: Vec<String>){
        let mut id = 0;
        for val in validators.into_iter(){
            self.validators.insert(val, id);
            id += 1;
        }

        self.view_number += 1
    }
}

