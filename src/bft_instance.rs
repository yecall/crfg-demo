use std::collections::HashMap;
use crate::message::BftMsgPkg;

#[derive(Default, Debug, PartialEq, Eq, Hash)]
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
    pre_prepare: BftMsgPkg, 
    prepares: Vec<BftMsgPkg>,
    commits: Vec<BftMsgPkg>,
}

impl BftInstance{
    pub fn new() -> BftInstance{ BftInstance{phase: BftMsgType::BftNone, pre_prepare: BftMsgPkg::new(), prepares: vec![], commits: vec![]} }
}

