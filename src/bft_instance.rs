use std::collections::HashMap;
use crate::message::MessageEnv;

#[derive(Default, Debug)]
pub struct Index{
    view_number: i64,
    sequence: i64,
}

#[derive(Debug)]
pub enum BftMsgType{
    BftNone,
    BftPrePrePared,
    BftPrepared,
    BftCommited,
    BftMax,
}

#[derive(Debug)]
pub struct BftInstance{
    phase: BftMsgType,
    pre_prepare: MessageEnv, 
    prepares: Vec<MessageEnv>,
    commits: Vec<MessageEnv>,
}

impl BftInstance{
    pub fn new() -> BftInstance{ BftInstance{phase: BftMsgType::BftNone, pre_prepare: MessageEnv::new(), prepares: vec![], commits: vec![]} }
}

