use crate::message::*;
use std::collections::HashMap;

pub struct ViewChangeInstance{
    view_number: i64,
    sequence: i64,
    viewchanges: HashMap<i8, ViewChange>,
    viewchange_pkgs: Box<BftMsgPkg>,
    pre_prepred_env_set: PreparedSet, // Last prepared related pre-prepared env
    view_change_round: u8,

    start_time: i64,
    last_propose_time: i64,
    end_time: i64,

    last_newview_time: i64,
    newview: NewView,
    new_view_round: u32
}
