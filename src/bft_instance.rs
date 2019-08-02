use std::collections::HashMap;
use crate::message::{BftMsgPkg, PrePrePare, Commit};

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct BftInstanceIndex {
    pub sequence: i64,
    pub view_number: i64,
}

impl BftInstanceIndex{
    pub fn new() -> BftInstanceIndex{
        BftInstanceIndex{
            sequence: 0,
            view_number: 0,
        }
    }
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

impl BftInstance{
    pub fn new() -> BftInstance{
        BftInstance{
            phase: BftMsgType::BftNone,
            pre_prepare: PrePrePare::new(),
            prepares: HashMap::new(),
            commits: HashMap::new(),
            pre_prepare_pkg: BftMsgPkg::new(),
            prepares_pkgs: vec![],
            commits_pkgs: vec![],
        }
    }
}
