use std::collections::HashMap;
use crate::message::{BftMsgPkg, PrePrePare, Commit};

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct Index{
    view_number: i64,
    sequence: i64,
}

#[derive(Debug)]
pub enum BftMsgType{
    BftPrePrePared,
    BftPrepared,
    BftCommited,
    BftNone,
}

#[derive(Debug)]
pub struct BftInstance{
    phase: BftMsgType,
    pre_prepare: PrePrePare,
    prepares: HashMap<i8, PrePrePare>,
    commits: HashMap<i8, Commit>,

    //in case of view change
    pre_prepare_pkg: BftMsgPkg,
    prepares_pkgs: Vec<BftMsgPkg>,
    commits_pkgs: Vec<BftMsgPkg>,
}


