use crate::message::*;
use std::collections::HashMap;

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct ViewChangeIndex{
    sequence: i64,
    digest: String
}

impl ViewChangeIndex{
    pub fn new() -> ViewChangeIndex{
        ViewChangeIndex{
            sequence: 0,
            digest: String::new()
        }
    }
}

#[derive(Debug)]
pub struct ViewChangeInstance{
    view_number: i64,
    sequence: i64,
    view_changes: HashMap<i8, ViewChange>,
    view_change_pkgs: Box<BftMsgPkg>,
    pre_prepared_env_set: PreparedSet, // Last prepared related pre-prepared env
    view_change_round: u8,

    start_time: i64,
    last_propose_time: i64,
    end_time: i64,

    last_new_view_time: i64,
    new_view: NewView,
    new_view_round: u32
}

impl ViewChangeInstance{
    pub fn new() -> ViewChangeInstance {
        ViewChangeInstance {
            view_number: 0,
            sequence: 0,
            view_changes: HashMap::new(),
            view_change_pkgs: Box::new(BftMsgPkg::new()),
            pre_prepared_env_set: PreparedSet::new(),
            view_change_round: 0,
            start_time: 0,
            last_propose_time: 0,
            end_time: 0,
            last_new_view_time: 0,
            new_view: NewView::new(),
            new_view_round: 0
        }
    }
}
