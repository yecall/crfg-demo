use crate::message::*;
use std::collections::HashMap;

#[derive(Default, Debug, PartialEq, Eq, Hash)]
pub struct ViewChangeIndex{
    sequence: i64,
    digest: String
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
