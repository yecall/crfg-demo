use std::collections::HashMap;
use crate::message::{BftMsgPkg, PrePrePare, Commit};

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct BftInstanceIndex {
    pub sequence: i64,
    pub view_number: i64,
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
    pub phase: BftMsgType,
    pub pre_prepare: PrePrePare,
    pub prepares: HashMap<i8, PrePrePare>,
    pub commits: HashMap<i8, Commit>,

    //in case of view change
    pub pre_prepare_pkg: BftMsgPkg,
    pub prepares_pkgs: Vec<BftMsgPkg>,
    pub commits_pkgs: Vec<BftMsgPkg>,
}
